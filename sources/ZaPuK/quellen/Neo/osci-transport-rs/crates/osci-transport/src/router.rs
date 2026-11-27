// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::dialog::ControlBlock;
use crate::error::{Error, Result};
use crate::message::MessageType;
use crate::parts::ContentContainer;
use crate::role::Role;

/// A boxed, `Send` future, as returned by handlers and layers.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A message handed to a handler: type, ControlBlock, first content container,
/// and the roles the crate could resolve.
#[derive(Clone, Debug, Default)]
pub struct IncomingMessage {
    /// The message type.
    pub ty: Option<MessageType>,
    /// The `osci:ControlBlock`.
    pub control_block: ControlBlock,
    /// The first plain content container, if the message carried one.
    pub container: Option<ContentContainer>,
    /// Resolved roles (originator, addressee, ...).
    pub roles: Vec<Role>,
    /// The MIME attachments, as `cid:` references point to them.
    pub attachments: Vec<crate::mime::Attachment>,
}

/// A handler's reply, before it is serialized into a response envelope.
#[derive(Clone, Debug)]
pub struct OutgoingMessage {
    /// The `responseTo*` message type.
    pub ty: MessageType,
    /// The response ControlBlock.
    pub control_block: crate::dialog::ControlBlock,
    /// Feedback entries for the response.
    pub feedback: Vec<crate::parts::Feedback>,
    /// The response content container, if any.
    pub container: Option<ContentContainer>,
}

impl OutgoingMessage {
    /// An empty response of the given type.
    pub fn new(ty: MessageType) -> Self {
        OutgoingMessage {
            ty,
            control_block: crate::dialog::ControlBlock::default(),
            feedback: Vec::new(),
            container: None,
        }
    }

    /// Set the response ControlBlock.
    pub fn control_block(mut self, cb: crate::dialog::ControlBlock) -> Self {
        self.control_block = cb;
        self
    }

    /// Append one feedback entry.
    pub fn feedback(mut self, fb: crate::parts::Feedback) -> Self {
        self.feedback.push(fb);
        self
    }

    /// Attach a content container.
    pub fn with_container(mut self, c: ContentContainer) -> Self {
        self.container = Some(c);
        self
    }
}

/// Anything a handler may return: an [`OutgoingMessage`] or a `Result` of one.
pub trait IntoOutgoing {
    /// Convert into the canonical handler result.
    fn into_outgoing(self) -> Result<OutgoingMessage>;
}

impl IntoOutgoing for OutgoingMessage {
    fn into_outgoing(self) -> Result<OutgoingMessage> {
        Ok(self)
    }
}

impl IntoOutgoing for Result<OutgoingMessage> {
    fn into_outgoing(self) -> Result<OutgoingMessage> {
        self
    }
}

/// A handler-argument extractor. Any implementor can appear in a handler's
/// parameter list.
pub trait FromMessage<S>: Sized {
    /// Pull this value out of the message (and the shared state).
    fn from_message(msg: &IncomingMessage, state: &S) -> Result<Self>;
}

/// Extractor for the router's shared state.
pub struct State<S>(pub S);

impl<S: Clone> FromMessage<S> for State<S> {
    fn from_message(_: &IncomingMessage, state: &S) -> Result<Self> {
        Ok(State(state.clone()))
    }
}

impl<S> FromMessage<S> for ControlBlock {
    fn from_message(msg: &IncomingMessage, _: &S) -> Result<Self> {
        Ok(msg.control_block.clone())
    }
}

impl<S> FromMessage<S> for ContentContainer {
    fn from_message(msg: &IncomingMessage, _: &S) -> Result<Self> {
        msg.container
            .clone()
            .ok_or_else(|| Error::MissingChild("ContentContainer".into()))
    }
}

impl<S> FromMessage<S> for IncomingMessage {
    fn from_message(msg: &IncomingMessage, _: &S) -> Result<Self> {
        Ok(msg.clone())
    }
}

/// An async function that answers a message.
///
/// Blanket-implemented for `Fn(A, B, ...) -> Future` where each argument is a
/// [`FromMessage`] extractor and the output is an [`IntoOutgoing`], for up to
/// three arguments.
pub trait Handler<T, S>: Clone + Send + Sync + Sized + 'static {
    /// Run the handler.
    fn call(
        self,
        msg: IncomingMessage,
        state: Arc<S>,
    ) -> BoxFuture<'static, Result<OutgoingMessage>>;
}

macro_rules! impl_handler {
    ($($arg:ident),*) => {
        impl<F, Fut, R, S, $($arg,)*> Handler<($($arg,)*), S> for F
        where
            F: Fn($($arg,)*) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = R> + Send + 'static,
            R: IntoOutgoing,
            S: Send + Sync + 'static,
            $($arg: FromMessage<S> + Send,)*
        {
            #[allow(non_snake_case, unused_variables)]
            fn call(self, msg: IncomingMessage, state: Arc<S>)
                -> BoxFuture<'static, Result<OutgoingMessage>>
            {
                Box::pin(async move {
                    $(let $arg = $arg::from_message(&msg, &state)?;)*
                    self($($arg,)*).await.into_outgoing()
                })
            }
        }
    };
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);

type Route<S> = Arc<
    dyn Fn(IncomingMessage, Arc<S>) -> BoxFuture<'static, Result<OutgoingMessage>> + Send + Sync,
>;

/// The rest of the pipeline, called by a [`Layer`] to continue processing.
pub type Next =
    Arc<dyn Fn(IncomingMessage) -> BoxFuture<'static, Result<OutgoingMessage>> + Send + Sync>;

/// Middleware wrapping the whole dispatch: verify a signature, enforce a dialog,
/// write a Laufzettel entry, map errors to a SOAPFault, and so on.
///
/// Call `next` to continue, or return your own [`OutgoingMessage`] to
/// short-circuit. Closures implement this trait directly.
pub trait Layer: Send + Sync + 'static {
    /// Process the message and optionally call `next`.
    fn call(&self, msg: IncomingMessage, next: Next)
    -> BoxFuture<'static, Result<OutgoingMessage>>;
}

impl<F> Layer for F
where
    F: Fn(IncomingMessage, Next) -> BoxFuture<'static, Result<OutgoingMessage>>
        + Send
        + Sync
        + 'static,
{
    fn call(
        &self,
        msg: IncomingMessage,
        next: Next,
    ) -> BoxFuture<'static, Result<OutgoingMessage>> {
        self(msg, next)
    }
}

/// Routes each [`MessageType`] to a [`Handler`], wrapped by any [`Layer`]s.
pub struct Router<S = ()> {
    state: Arc<S>,
    routes: HashMap<MessageType, Route<S>>,
    layers: Vec<Arc<dyn Layer>>,
}

impl<S: Send + Sync + 'static> Router<S> {
    /// A router with shared state and no routes.
    pub fn new(state: S) -> Self {
        Router {
            state: Arc::new(state),
            routes: HashMap::new(),
            layers: Vec::new(),
        }
    }

    /// Register `handler` for one message type.
    pub fn on<T, H>(mut self, mt: MessageType, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let route: Route<S> = Arc::new(move |msg, state| handler.clone().call(msg, state));
        self.routes.insert(mt, route);
        self
    }

    /// Add a layer. Layers run outermost-first, in the order they were added.
    pub fn layer(mut self, layer: impl Layer) -> Self {
        self.layers.push(Arc::new(layer));
        self
    }

    // Run the layers, then resolve and call the handler for the message type.
    // Layers see the message before route resolution, so a layer can answer or
    // reject an unknown or unwanted message type itself.
    pub async fn dispatch(&self, msg: IncomingMessage) -> Result<OutgoingMessage> {
        let routes = self.routes.clone();
        let state = self.state.clone();
        let mut next: Next = Arc::new(move |msg: IncomingMessage| {
            let routes = routes.clone();
            let state = state.clone();
            Box::pin(async move {
                let mt = msg
                    .ty
                    .ok_or_else(|| Error::msg("incoming message has no type"))?;
                let route = routes
                    .get(&mt)
                    .ok_or_else(|| Error::msg(format!("no handler for {mt:?}")))?
                    .clone();
                route(msg, state).await
            })
        });
        for layer in self.layers.iter().rev() {
            let inner = next.clone();
            let layer = layer.clone();
            next = Arc::new(move |msg| layer.call(msg, inner.clone()));
        }
        next(msg).await
    }
}

impl Default for Router<()> {
    fn default() -> Self {
        Router::new(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn block_on<F: Future>(mut f: F) -> F::Output {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VT)
        }
        static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VT)) };
        let mut cx = Context::from_waker(&waker);
        let mut f = unsafe { Pin::new_unchecked(&mut f) };
        loop {
            if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
                return v;
            }
        }
    }

    #[derive(Clone)]
    struct AppState {
        name: String,
    }

    async fn handle(State(s): State<AppState>, msg: IncomingMessage) -> OutgoingMessage {
        assert_eq!(s.name, "test");
        let _ = msg;
        OutgoingMessage::new(MessageType::ResponseToStoreDelivery)
    }

    #[test]
    fn dispatches_to_handler_with_extractors() {
        let router = Router::new(AppState {
            name: "test".into(),
        })
        .on(MessageType::StoreDelivery, handle);

        let incoming = IncomingMessage {
            ty: Some(MessageType::StoreDelivery),
            ..Default::default()
        };
        let out = block_on(router.dispatch(incoming)).unwrap();
        assert_eq!(out.ty, MessageType::ResponseToStoreDelivery);
    }

    #[test]
    fn layers_wrap_the_handler_outermost_first() {
        use std::sync::Mutex;
        use std::sync::atomic::{AtomicUsize, Ordering};

        static ORDER: Mutex<Vec<&str>> = Mutex::new(Vec::new());
        let counter = Arc::new(AtomicUsize::new(0));

        async fn handle() -> OutgoingMessage {
            ORDER.lock().unwrap().push("handler");
            OutgoingMessage::new(MessageType::ResponseToExitDialog)
        }

        let c = counter.clone();
        let router = Router::default()
            .on(MessageType::ExitDialog, handle)
            .layer(move |msg: IncomingMessage, next: Next| {
                let c = c.clone();
                Box::pin(async move {
                    ORDER.lock().unwrap().push("outer-before");
                    c.fetch_add(1, Ordering::SeqCst);
                    let r = next(msg).await;
                    ORDER.lock().unwrap().push("outer-after");
                    r
                }) as BoxFuture<'static, Result<OutgoingMessage>>
            })
            .layer(|msg: IncomingMessage, next: Next| {
                Box::pin(async move {
                    ORDER.lock().unwrap().push("inner-before");
                    let r = next(msg).await;
                    ORDER.lock().unwrap().push("inner-after");
                    r
                }) as BoxFuture<'static, Result<OutgoingMessage>>
            });

        let incoming = IncomingMessage {
            ty: Some(MessageType::ExitDialog),
            ..Default::default()
        };
        let out = block_on(router.dispatch(incoming)).unwrap();
        assert_eq!(out.ty, MessageType::ResponseToExitDialog);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(
            *ORDER.lock().unwrap(),
            [
                "outer-before",
                "inner-before",
                "handler",
                "inner-after",
                "outer-after"
            ]
        );
    }

    #[test]
    fn a_layer_can_short_circuit() {
        async fn never() -> OutgoingMessage {
            panic!("handler must not run");
        }
        let router = Router::default().on(MessageType::ExitDialog, never).layer(
            |_msg: IncomingMessage, _next: Next| {
                Box::pin(async { Ok(OutgoingMessage::new(MessageType::ResponseToExitDialog)) })
                    as BoxFuture<'static, Result<OutgoingMessage>>
            },
        );
        let incoming = IncomingMessage {
            ty: Some(MessageType::ExitDialog),
            ..Default::default()
        };
        let out = block_on(router.dispatch(incoming)).unwrap();
        assert_eq!(out.ty, MessageType::ResponseToExitDialog);
    }

    #[test]
    fn unknown_route_errors() {
        let router = Router::default();
        let incoming = IncomingMessage {
            ty: Some(MessageType::ExitDialog),
            ..Default::default()
        };
        assert!(block_on(router.dispatch(incoming)).is_err());
    }
}
