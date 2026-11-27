// SPDX-FileCopyrightText: 2025 Element
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {
	BackgroundTaskFailureReason_Tags,
	BackupDownloadStrategy,
	ClientBuilder,
	type ClientDelegate,
	type ClientLike,
	type ClientSessionDelegate,
	IndexedDbStoreBuilder,
	LogLevel,
	MediaSource,
	Membership,
	type MessageServiceLike,
	OAuthConfiguration,
	OAuthPrompt,
	type RoomListServiceLike,
	Session,
	SlidingSyncVersionBuilder,
	type SyncServiceLike,
	SyncServiceState,
	type SyncServiceStateObserver,
	messageService as createMessageService,
	initPlatform,
} from '../index.web';
import CaseStore from './CaseStore';
import RoomListStore from './RoomListStore';
import TimelineStore from './TimelineStore';
import { makeBrandedType, printRustError } from './utils';

/**
 * Parameters required for user authentication and login.
 *
 * @interface LoginParams
 * @property {string} username - The user's login identifier
 * @property {string} password - The user's authentication password
 * @property {string} server - The homeserver URL or identifier to connect to
 */
interface LoginParams {
	username: string;
	password: string;
	server: string;
}

/**
 * Config required for the OIDC authorization process.
 *
 * @interface OidcConfig
 * @property clientName The name shown to users during OIDC authorization. Defaults to 'Meine Digitale Verwaltung'.
 * @property clientUri URI of this client, shown during OIDC authorization. Defaults to window.location.origin.
 */
interface OidcConfig {
	clientName?: string;
	clientUri?: string;
}

/**
 * Config for the matrix client.
 *
 * @interface ClientConfig
 * @property matrixHomeserver Base URL of the matrix homeserver to use
 * @property serviceWorkerUrl The pathname for the service worker URL to use. Defaults to '/sw.js'.
 * @property dbName Name of the IndexedDB database to use. Defaults to 'matrix-store'.
 * @property oidc The config for the OIDC authorization process.
 * @property onLogout Cleanup awaited before logout navigates away. Failures are logged and do not prevent logout.
 */
export interface ClientConfig {
	matrixHomeserver: string;
	serviceWorkerUrl?: string;
	dbName?: string;
	oidc?: OidcConfig;
	onLogout?: () => void | Promise<void>;
}

/**
 * Represents the different states a client can be in during the authentication lifecycle.
 *
 * @remarks
 * This branded type ensures type safety when working with client states throughout the application.
 * The states follow a logical progression from unknown through various authentication phases.
 *
 * @public
 */
export const ClientState = makeBrandedType(
	{
		Unknown: 'Unknown',
		LoadingSession: 'LoadingSession',
		LoggedOut: 'LoggedOut',
		LoggingOut: 'LoggingOut',
		LoggingIn: 'LoggingIn',
		LoggedIn: 'LoggedIn',
	},
	'ClientState',
);

/**
 * Type alias that extracts the union type of all values from the ClientState object.
 * This creates a type that represents all possible client state values.
 */
export type ClientState = (typeof ClientState)[keyof typeof ClientState];

/**
 * Stores Matrix client session (username, token, etc) in localStorage
 */
class SessionStore {
	/**
	 * Loads a session from localStorage.
	 *
	 * Retrieves the stored session data from localStorage under the key "mx_session",
	 * parses it from JSON, and creates a new Session instance if valid data exists.
	 *
	 * @returns The loaded Session instance if valid session data exists in localStorage,
	 *          otherwise undefined
	 */
	load(): Session | undefined {
		const stored = localStorage.getItem('mx_session');
		const session = stored ? JSON.parse(stored) : undefined;
		return session ? Session.new(session) : undefined;
	}
	/**
	 * Saves a session object to local storage.
	 *
	 * @param session - The session object to be saved
	 * @returns void
	 */
	save(session: Session): void {
		localStorage.setItem('mx_session', JSON.stringify(session));
	}

	/**
	 * Clears the stored session data by removing the matrix session from local storage.
	 * This effectively logs out the user and removes all cached session information.
	 */
	clear(): void {
		localStorage.removeItem('mx_session');
	}
}

/**
 * A centralized store for managing Matrix client operations, authentication, and synchronization.
 *
 * The ClientStore handles the complete lifecycle of a Matrix client including:
 * - User authentication (login/logout)
 * - Session management and restoration
 * - Client synchronization with the homeserver
 * - Service worker registration for background operations
 * - Room list management
 * - State management with observer pattern for UI updates
 *
 * @example
 * ```typescript
 * const store = new ClientStore();
 *
 * // Subscribe to state changes
 * const unsubscribe = store.subscribe(() => {
 *   console.log('Client state:', store.getSnapshot());
 * });
 *
 * // Login
 * await store.login({
 *   username: 'user@example.com',
 *   password: 'password',
 *   server: 'https://matrix.org'
 * });
 *
 * // Get room list
 * const roomListStore = store.getRoomListStore();
 * ```
 */
class ClientStore {
	sessionStore = new SessionStore();

	constructor(private readonly config: ClientConfig) {}

	timelineStore?: TimelineStore;
	caseStore?: CaseStore;
	private readonly joiningRooms = new Set<string>();
	private readonly joinFailedRooms = new Set<string>();
	roomListStore?: RoomListStore;
	client?: ClientLike;
	syncService?: SyncServiceLike;
	roomListService?: RoomListServiceLike;
	private messageServiceLike?: MessageServiceLike;

	/** Holds the client built during startOidcLogin() until completeOidcLogin() is called. */
	private pendingOidcClient?: ClientLike;

	clientState: ClientState = ClientState.Unknown;
	listeners: CallableFunction[] = [];

	/**
	 * Registers a service worker primarily used for authenticated media fetching.
	 */
	private async registerServiceWorker() {
		const registration = await navigator.serviceWorker.register(
			this.config.serviceWorkerUrl ?? '/sw.js',
		);
		if (!registration) {
			throw new Error('Service worker registration failed');
		}
		navigator.serviceWorker.addEventListener(
			'message',
			this.onServiceWorkerPostMessage,
		);
		// Re-push auth data whenever the SW becomes the active controller (e.g. after update).
		navigator.serviceWorker.addEventListener(
			'controllerchange',
			this.pushAuthDataToServiceWorker,
		);
		await registration.update();
		// Push immediately in case the SW is already controlling this page.
		this.pushAuthDataToServiceWorker();
	}

	/**
	 * Sends the current session's auth data to the service worker so it can
	 * authenticate media requests even when event.clientId is unavailable (Safari).
	 */
	private pushAuthDataToServiceWorker = () => {
		if (!this.client) return;
		const controller = navigator.serviceWorker.controller;
		if (!controller) return;
		try {
			controller.postMessage({
				type: 'setAuthData',
				accessToken: this.client.session().accessToken,
				homeserver: this.client.homeserver(),
			});
		} catch (e) {
			console.warn(
				'[ClientStore] Failed to push auth data to service worker',
				e,
			);
		}
	};

	/**
	 * Handles incoming messages from a service worker requesting user information.
	 *
	 * When the service worker sends a message with type "userinfo", this method responds
	 * with the current user's access token and homeserver URL. The response is sent back
	 * to the service worker using the provided response key for correlation.
	 *
	 * @param event - The MessageEvent received from the service worker containing the request data
	 *
	 * @remarks
	 * - Only processes messages from the same origin as the current window
	 * - Requires an active client session to respond with user information
	 * - Uses responseKey from the request to correlate the response
	 * - Logs errors to console if message processing fails
	 */
	private onServiceWorkerPostMessage = (event: MessageEvent): void => {
		if (!this.client) return;
		try {
			const data = event.data as { type?: string; responseKey?: string };
			if (
				event.origin === window.origin &&
				data?.type === 'userinfo' &&
				data?.responseKey
			) {
				const accessToken = this.client.session().accessToken;
				const homeserver = this.client.homeserver();
				event.source?.postMessage({
					responseKey: data.responseKey,
					accessToken,
					homeserver,
				});
			}
		} catch (e) {
			console.error('Error responding to service worker: ', e);
		}
	};

	/**
	 * Creates and returns a new ClientBuilder instance configured with sliding sync version discovery.
	 *
	 * The returned ClientBuilder is pre-configured to use native sliding sync version discovery,
	 * which automatically detects and uses the appropriate sliding sync implementation.
	 *
	 * @returns A new ClientBuilder instance with sliding sync version builder set to discover native implementation
	 */
	getClientBuilder() {
		const sessionDelegate: ClientSessionDelegate = {
			retrieveSessionFromKeychain: (userId: string) => {
				const session = this.sessionStore.load();
				if (!session) throw new Error(`No session for ${userId}`);
				return session;
			},
			saveSessionInKeychain: (session: Session) => {
				this.sessionStore.save(session);
			},
		};
		return new ClientBuilder()
			.indexeddbStore(
				new IndexedDbStoreBuilder(this.config.dbName ?? 'matrix-store'),
			)
			.slidingSyncVersionBuilder(SlidingSyncVersionBuilder.DiscoverNative)
			.autoEnableBackups(true)
			.autoEnableCrossSigning(true)
			.backupDownloadStrategy(BackupDownloadStrategy.AfterDecryptionFailure)
			.setSessionDelegate(sessionDelegate);
	}

	/**
	 * Builds the OidcConfiguration from the app config.
	 */
	private getOidcConfiguration(): OAuthConfiguration {
		return OAuthConfiguration.create({
			clientName: this.config.oidc?.clientName ?? 'Meine Digitale Verwaltung',
			redirectUri: `${window.location.origin}/login/callback`,
			clientUri: this.config.oidc?.clientUri ?? window.location.origin,
			logoUri: undefined,
			tosUri: undefined,
			policyUri: undefined,
			staticRegistrations: new Map(),
		});
	}

	/**
	 * Initiates the OIDC login flow by building a client, requesting an
	 * authorization URL from the homeserver, and returning it for the popup.
	 *
	 * **Important:** The client built here is kept alive in `pendingOidcClient`
	 * because the PKCE verifier is stored in-memory on the client object.
	 * `completeOidcLogin` must be called on the same JS runtime (i.e. the same
	 * window/tab) — use a popup so the main window is never reloaded.
	 *
	 * @returns The authorization URL to open in a popup.
	 */
	async startOidcLogin(): Promise<string> {
		this.clientState = ClientState.LoggingIn;
		this.emit();

		const client = await this.getClientBuilder()
			.homeserverUrl(this.config.matrixHomeserver)
			.build();

		try {
			initPlatform(
				{
					logLevel: LogLevel.Trace,
					traceLogPacks: [],
					extraTargets: [],
					writeToStdoutOrSystem: true,
					writeToFiles: undefined,
				},
				true,
			);

			const authorizationData = await client.urlForOauth(
				this.getOidcConfiguration(),
				OAuthPrompt.Login.new(),
				undefined,
				undefined,
				undefined,
			);
			// Keep this client instance alive — the PKCE verifier is in its memory.
			this.pendingOidcClient = client;
			return authorizationData.loginUrl();
		} catch (e) {
			printRustError('startOidcLogin failed', e);
			this.clientState = ClientState.LoggedOut;
			this.emit();
			throw e;
		}
	}

	/**
	 * Completes the OIDC login flow using the callback URL received from the
	 * authorization server. Must be called in the same JS context (window) as
	 * `startOidcLogin` so that the in-memory PKCE state is available.
	 *
	 * @param callbackUrl - The full URL of the `/login/callback` page including
	 *   query parameters (`code`, `state`, etc.).
	 */
	async completeOidcLogin(callbackUrl: string): Promise<void> {
		const client = this.pendingOidcClient;
		if (!client) {
			throw new Error(
				'No pending OIDC login — startOidcLogin() must be called first in the same window.',
			);
		}
		this.pendingOidcClient = undefined;

		this.clientState = ClientState.LoggingIn;
		this.emit();

		try {
			await client.loginWithOauthCallback(callbackUrl);
			console.log('OIDC login complete');

			this.sessionStore.save(client.session());
			this.client = client;
			this.clientState = ClientState.LoggedIn;
		} catch (e) {
			printRustError('completeOidcLogin failed', e);

			if (String(e).includes("account in the store doesn't match")) {
				// The crypto store has a stale device ID from a previous session.
				// We cannot delete the DB here — the local `client` variable is still
				// in scope and its WASM object holds the connections open, so
				// deleteDatabase would block forever.
				//
				// Strategy: clear the saved session from localStorage NOW (so that
				// after the reload tryLoadSession() does nothing and cannot restore
				// the old device ID back into the freshly-cleared DB), flag the DB
				// clear for main.tsx, then reload.
				this.sessionStore.clear();
				sessionStorage.setItem('fit-neo-needs-db-clear', '1');
				sessionStorage.setItem('fit-neo-login-retry', '1');
				window.location.reload();
				// Don't update state or throw — the page is going away anyway.
				return;
			}

			this.clientState = ClientState.LoggedOut;
			this.emit();
			throw e;
		}

		await this.sync();
		this.emit();
	}

	/**
	 * Attempts to load and restore a previously saved client session.
	 *
	 * This method tries to restore a Matrix client session from the session store.
	 * If successful, it restores the client and initiates synchronization.
	 * If no session is found or restoration fails, the client state is set to logged out.
	 *
	 * @throws {Error} When no session is found in the session store
	 * @returns {Promise<void>} A promise that resolves when the session loading attempt is complete
	 *
	 * @remarks
	 * - Sets client state to `LoadingSession` at the start
	 * - On success: restores client, sets state to `LoggedIn`, and starts sync
	 * - On failure: logs the error, sets state to `LoggedOut`
	 * - Emits state changes throughout the process
	 */
	async tryLoadSession(): Promise<void> {
		this.clientState = ClientState.LoadingSession;
		this.emit();
		try {
			const session = this.sessionStore.load();
			if (!session) {
				throw new Error('No session found');
			}

			const client = await this.getClientBuilder()
				.homeserverUrl(session.homeserverUrl)
				.build();
			this.client = client;
			this.setupClientDelegate(client);
			await client.restoreSession(session);
			this.clientState = ClientState.LoggedIn;
		} catch (e) {
			printRustError('Failed to restore session', e);
			this.clientState = ClientState.LoggedOut;
			this.emit();
			return;
		}

		await this.sync();
		this.emit();
	}

	/**
	 * Logs out the current user and cleans up all client-related resources.
	 *
	 * This method performs a complete logout by:
	 * - Clearing the session store
	 * - Resetting the Matrix client instance
	 * - Cleaning up room list store and service
	 * - Stopping the sync service
	 * - Setting client state to logged out
	 * - Emitting state change event
	 */
	logout(): void {
		// Guard against double-calling (e.g. React StrictMode effect double-invocation).
		if (
			this.clientState === ClientState.LoggingOut ||
			this.clientState === ClientState.LoggedOut
		) {
			return;
		}

		const syncService = this.syncService;
		const client = this.client;

		// Clear the saved session immediately so tryLoadSession() on the next page
		// load does not restore a stale session into the freshly-wiped DB.
		this.sessionStore.clear();
		// Tell main.tsx to wipe all three Matrix IndexedDB stores before any WASM
		// is initialised on the next load. Because no connections exist at that
		// point, deleteDatabase always succeeds immediately (no onblocked).
		sessionStorage.setItem('fit-neo-needs-db-clear', '1');

		navigator.serviceWorker.removeEventListener(
			'message',
			this.onServiceWorkerPostMessage,
		);
		navigator.serviceWorker.removeEventListener(
			'controllerchange',
			this.pushAuthDataToServiceWorker,
		);
		this.client = undefined;
		this.timelineStore = undefined;
		this.caseStore = undefined;
		this.messageServiceLike = undefined;
		this.roomListStore = undefined;
		this.roomListService = undefined;
		this.syncService = undefined;
		this.joiningRooms.clear();
		this.joinFailedRooms.clear();
		this.clientState = ClientState.LoggingOut;
		this.emit();

		void (async () => {
			if (this.config.onLogout) {
				try {
					await this.config.onLogout();
				} catch (error) {
					console.error('Failed to run logout cleanup:', error);
				}
			}
			if (client) {
				try {
					client.setDelegate(undefined);
				} catch {
					// ignore
				}
			}
			if (syncService) {
				try {
					// Cap at 5 s — we are about to navigate away regardless.
					await Promise.race([
						syncService.stop(),
						new Promise<void>((r) => setTimeout(r, 5_000)),
					]);
				} catch (e) {
					printRustError('Failed to stop sync service on logout', e);
				}
			}
			// Hard-navigate to /login. The page reload releases all WASM/IndexedDB
			// connections, then main.tsx clears the stores before any WASM init —
			// deleteDatabase succeeds immediately because no connections are open.
			window.location.href = '/login';
		})();
	}

	/**
	 * Authenticates a user with the Matrix homeserver and initializes the client session.
	 *
	 * @param params - The login parameters
	 * @param params.username - The username for authentication
	 * @param params.password - The password for authentication
	 * @param params.server - The homeserver URL to connect to
	 *
	 * @remarks
	 * This method performs the following operations:
	 * - Sets client state to logging in
	 * - Builds and configures the Matrix client
	 * - Initializes the platform with trace logging
	 * - Authenticates with the homeserver using username/password
	 * - Saves the session to persistent storage
	 * - Synchronizes with the homeserver
	 * - Updates client state and emits change events
	 *
	 * @throws Will log errors and set client state to Unknown if login fails
	 *
	 * @example
	 * ```typescript
	 * await clientStore.login({
	 *   username: 'user@example.com',
	 *   password: 'securePassword',
	 *   server: 'https://matrix.example.com'
	 * });
	 * ```
	 */
	async login({ username, password, server }: LoginParams) {
		this.clientState = ClientState.LoggingIn;
		this.emit();
		const client = await this.getClientBuilder().homeserverUrl(server).build();
		this.client = client;
		this.setupClientDelegate(client);

		console.log('starting sdk...');
		try {
			initPlatform(
				{
					logLevel: LogLevel.Trace,
					traceLogPacks: [],
					extraTargets: [],
					writeToStdoutOrSystem: true,
					writeToFiles: undefined,
				},
				true,
			);

			await client.login(username, password, 'fit-neo-sdk', undefined);
			console.log('logged in...');

			this.sessionStore.save(client.session());

			this.clientState = ClientState.LoggedIn;
		} catch (e) {
			printRustError('login failed', e);
			this.clientState = ClientState.Unknown;
			this.emit();
			return;
		}

		await this.sync();
		this.emit();
	}

	/**
	 * Synchronizes the Matrix client by initializing and starting the sync service.
	 *
	 * This method performs the following operations:
	 * 1. Registers a service worker for authenticated media fetching
	 * 2. Creates and configures a sync service with offline mode enabled which allows automatic revovery from network interruptions
	 * 3. Initializes the room list service from the sync service
	 * 4. Starts the synchronization process
	 * 5. Emits an event to notify listeners of successful sync
	 *
	 * @private
	 * @async
	 * @returns {Promise<void>} A promise that resolves when sync is complete
	 * @throws Will log errors and set client state to Unknown if sync fails
	 */
	private setupClientDelegate(client: ClientLike): void {
		const clientDelegate: ClientDelegate = {
			didReceiveAuthError: (isSoftLogout: boolean) => {
				console.warn(
					'[ClientStore] Auth error received, isSoftLogout:',
					isSoftLogout,
					'— forcing logout',
				);
				// TODO: Before logging out, show a modal/page prompting the user to
				// verify their recovery key or export their crypto keys so that Olm
				// session data is not lost. Without this, keys stored only in the
				// local IndexedDB become irrecoverable after the DB is wiped on the
				// next load. This should be gated on the secure key storage feature
				// (design TBD by UX team).
				void this.logout();
			},
			onBackgroundTaskErrorReport: (taskName, error) => {
				if (error.tag === BackgroundTaskFailureReason_Tags.Panic) {
					console.error(
						`[ClientStore] Background task "${taskName}" panicked:`,
						error.inner.message,
						error.inner.panicBacktrace,
					);
				} else if (error.tag === BackgroundTaskFailureReason_Tags.Error) {
					console.warn(
						`[ClientStore] Background task "${taskName}" failed:`,
						error.inner.error,
					);
				}
			},
		};
		client.setDelegate(clientDelegate);
	}

	private async sync(): Promise<void> {
		this.registerServiceWorker();

		try {
			const syncServiceBuilder = this.client!.syncService();
			this.syncService = await syncServiceBuilder.withOfflineMode().finish();
			this.roomListService = this.syncService.roomListService();

			// Watch for terminal sync states as a fallback. In offline mode the
			// SDK keeps serving cached data even during auth failures, so we also
			// catch Error (unrecoverable) in addition to Terminated.
			const stateObserver: SyncServiceStateObserver = {
				onUpdate: (state: SyncServiceState) => {
					if (
						state === SyncServiceState.Terminated ||
						state === SyncServiceState.Error
					) {
						console.warn(
							`[ClientStore] Sync service reached terminal state ${state} — forcing logout`,
						);
						// TODO: Same as in didReceiveAuthError — show a crypto key
						// backup/recovery prompt before wiping the session (see comment
						// above). Gated on the secure storage feature (UX TBD).
						void this.logout();
					}
				},
			};
			this.syncService.state(stateObserver);

			await this.syncService.start();
			console.log('syncing...');
			this.emit();
		} catch (e) {
			printRustError('syncing failed', e);
			this.clientState = ClientState.Unknown;
			return;
		}
	}

	/**
	 * Subscribes a listener function to be notified of store changes.
	 *
	 * @param listener - The callback function to be invoked when the store state changes
	 * @returns A function that when called, unsubscribes the listener from the store
	 *
	 * @example
	 * ```typescript
	 * const unsubscribe = store.subscribe((newState) => {
	 *   console.log('State changed:', newState);
	 * });
	 *
	 * // Later, to unsubscribe:
	 * unsubscribe();
	 * ```
	 */
	subscribe(listener: CallableFunction): () => void {
		this.listeners = [...this.listeners, listener];
		return () => {
			this.listeners = this.listeners.filter((l) => l !== listener);
		};
	}

	/**
	 * Returns a snapshot of the current client state.
	 *
	 * @returns The current client state object
	 */
	getSnapshot(): ClientState {
		return this.clientState;
	}

	getHomeserver(): string | undefined {
		return this.client?.homeserver();
	}

	async getMediaContent(
		uri: string,
		signal?: AbortSignal,
	): Promise<ArrayBuffer> {
		if (!this.client) throw new Error('Not logged in');
		const source = MediaSource.fromUrl(uri);
		return this.client.getMediaContent(source, signal ? { signal } : undefined);
	}

	async uploadMedia(data: ArrayBuffer, mimeType: string): Promise<string> {
		if (!this.client) throw new Error('Not logged in');
		return this.client.uploadMedia(mimeType, data, undefined);
	}

	/**
	 * Notifies all registered listeners by invoking their callback functions.
	 * This method iterates through the listeners array and calls each listener function.
	 *
	 * @private
	 */
	private emit() {
		for (const listener of this.listeners) {
			listener();
		}
	}

	getTimelineStore(roomId: string): TimelineStore | undefined {
		if (roomId === '') return;
		if (this.timelineStore?.room.id() !== roomId) {
			this.timelineStore?.stop();
			const room = this.client?.getRoom(roomId);
			if (!room) {
				console.warn('Room not found:', roomId);
				return;
			}
			console.log('getting timeline store for room', roomId, room);
			const store = new TimelineStore(room!);
			store.run();
			this.timelineStore = store;
		}
		return this.timelineStore;
	}

	hasJoinFailed(caseId: string): boolean {
		return this.joinFailedRooms.has(caseId);
	}

	getCaseStore(caseId: string): CaseStore | undefined {
		if (!caseId || !this.client) return undefined;
		if (this.caseStore?.caseId !== caseId) {
			this.caseStore?.stop();
			if (!this.messageServiceLike) {
				this.messageServiceLike = createMessageService(this.client);
			}
			const room = this.client.getRoom(caseId);
			if (!room) return undefined;
			if (room.membership() === Membership.Invited) {
				if (this.joinFailedRooms.has(caseId)) return undefined;
				if (!this.joiningRooms.has(caseId)) {
					this.joiningRooms.add(caseId);
					room
						.join()
						.then(() => {
							this.joiningRooms.delete(caseId);
							this.emit();
						})
						.catch((e) => {
							this.joiningRooms.delete(caseId);
							this.joinFailedRooms.add(caseId);
							printRustError(`Failed to join room ${caseId}`, e);
							this.emit();
						});
				}
				return undefined;
			}
			const store = new CaseStore(this.messageServiceLike, room, caseId);
			store.run();
			this.caseStore = store;
		}
		return this.caseStore;
	}

	/**
	 * Gets or creates a RoomListStore instance.
	 *
	 * Creates a new RoomListStore if one doesn't exist and both roomListService
	 * and syncService are available. The store is automatically started when created.
	 *
	 * @returns The RoomListStore instance, or undefined if required services are not available
	 */
	getRoomListStore(): RoomListStore | undefined {
		if (!this.roomListStore && this.roomListService && this.syncService) {
			const store = new RoomListStore(this.roomListService!);
			store.run();
			this.roomListStore = store;
		}
		return this.roomListStore;
	}
}

export default ClientStore;
