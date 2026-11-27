<!--
SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# osci-transport-ffi

UniFFI bindings for the parts of OSCI-Transport that client applications need:
classifying an OSCI message, decrypting the transport layer, decrypting a
content container, signing a request, building a reply, and handling
attachments (the non-XML MIME parts of a message).

Attachments are read with `message_attachments` and written with
`wrap_message_with_attachments`. `encrypt_container_with_attachments` and
`decrypt_content_container_with_attachments` encrypt and decrypt them together
with their container, and `verify_attachment_digests` checks received parts
against the digests of the message signature. `sign_request` signs an unsigned
request envelope as the originator, covering the attachments, and
`cert_slot_id` gives the header Id of a certificate for the content encryption
functions. `envelope_certificates` lists the certificates in the header of a
received message, such as the sender's cipher certificate a reply is encrypted
for. `decrypt_message_content` reads a whole received message in one call: it
splits the MIME, decrypts the content addressed to the given key and the
attachments that go with it. `build_reply_payload` builds the payload of a
reply to a received message: contents and attachments encrypted for the
original sender and, if given, for the author's own cipher certificate,
without signature or envelope. `generate_cipher_key`
generates an RSA key and a PKCS#10 certificate request for it, to get a cipher
certificate. The private key comes as PKCS#8 PEM, the form the decrypt
functions take.

The exported surface is defined with UniFFI proc-macros in `src/lib.rs`.

## Generating bindings

TypeScript bindings for React Native, the web (WASM) and Node.js are generated
with [`uniffi-bindgen-react-native`](https://github.com/jhugman/uniffi-bindgen-react-native)
(`ubrn`), which consumes this crate:

```sh
cargo build -p osci-transport-ffi --release
ubrn generate bindings --crate osci-transport-ffi
```

Swift bindings for native iOS use the standard `uniffi-bindgen`.

## Browser package

`web/` builds this crate to WebAssembly with generated TypeScript bindings and
publishes it as `@matrix-g2x/osci-transport-web` to the GitLab npm registry of
this project. Import it from `@matrix-g2x/osci-transport-web/web`. Releases
come from `main`, and every `develop` build is published under the `next` tag.

To build it locally, install the rustup target `wasm32-unknown-unknown` and a
`wasm-bindgen` CLI that matches the version in `Cargo.lock`, then run
`yarn install && ./build.sh` in `web/`. Set `UBRN_WASM_BINDGEN` if the CLI is
not on `PATH`.
