// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

// Writes fixtures the Java interop harness (tests/interop/java) checks against
// the official Java osci-bibliothek.
//
// Rust-reads-Java is already covered by the vendored corpus. The natural
// counterpart, Rust-produces-a-request-and-Java-verifies-it, turns out to be
// unreachable: the official Java osci-bibliothek's only public parse entry point,
// PassiveRecipientParser, is scoped to the two request types an Intermediär
// forwards to a passive recipient (AcceptDelivery/ProcessDelivery) plus
// response types, and its own request-signing path (OSCIRequest.transmit)
// has no public way to intercept the bytes before they're posted.
//
// So these fixtures cover the direction that *is* reachable: a
// Supplier-side response, produced entirely by osci-transport-server, fed to
// an official Java osci-bibliothek client through a stub TransportI that returns it as
// the answer to a StoreDelivery it sends to itself (see
// RustFixturesTest.java). That's a genuine Java-verifies-Rust check of
// respond_with's MIME assembly and sign_envelope's SupplierSignature.
//
// Run with: cargo run -p osci-transport-crypto --example gen_interop_fixtures

use std::fs;
use std::path::PathBuf;

use osci_transport::crypto::{SignatureAlg, SymmetricAlg};
use osci_transport::dialog::ControlBlock;
use osci_transport::message::MessageType;
use osci_transport::mime::{Attachment, wrap_envelope_with};
use osci_transport::parts::{Content, ContentContainer, ContentKind, ContentPackage, Feedback};
use osci_transport::router::OutgoingMessage;
use osci_transport::wire::{ResponseEnvelope, encode_content_container_bytes};
use osci_transport_crypto::{
    CertSlot, EncryptOptions, Role, encrypt_attachment, encrypt_content_for_slot,
    sign_envelope_with_attachments,
};
use osci_transport_server::{ResponseOptions, respond_with};

fn key(p: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/interop/keys")
            .join(p),
    )
    .unwrap()
}

fn write_fixture(name: &str, bytes: &[u8]) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/interop/generated");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    fs::write(&path, bytes).unwrap();
    println!("wrote {} ({} bytes)", path.display(), bytes.len());
}

fn response() -> OutgoingMessage {
    OutgoingMessage {
        ty: MessageType::ResponseToStoreDelivery,
        control_block: ControlBlock {
            id: Some("controlblock".into()),
            conversation_id: Some("interop".into()),
            response: Some("9010101".into()),
            sequence_number: Some(0),
            ..Default::default()
        },
        feedback: vec![Feedback {
            code: "9010101".into(),
            text: "Auftrag ausgefuehrt.".into(),
            lang: "de".into(),
        }],
        container: None::<ContentContainer>,
    }
}

fn main() {
    let manager_key = key("test_osci-manager_cypher.key.pem");
    let manager_cert = key("test_osci-manager_cypher.cer");

    // 1. an unsigned response: exercises respond_with's plain MIME assembly.
    let bytes = respond_with(&response(), &ResponseOptions::default()).unwrap();
    write_fixture("unsigned_response.osci", &bytes);

    // 2. the same, signed as Supplier: exercises sign_envelope against the
    //    real library's SupplierSignature verification.
    let opts = ResponseOptions {
        signer: Some((manager_key.clone(), manager_cert.clone())),
        encrypt_for: None,
    };
    let bytes = respond_with(&response(), &opts).unwrap();
    write_fixture("signed_response.osci", &bytes);

    // 3. a ResponseToFetchDelivery with one attachment, signed as Supplier. The
    //    signature holds the part's digest, which Java recomputes.
    let reader_cert = key("Reader01.cer");
    let pdf = Attachment {
        content_id: Some("Klage.pdf".into()),
        content_type: Some("application/pdf".into()),
        data: b"%PDF-1.7\r\n\x00\x01\xff binary body".to_vec(),
    };
    let mut cc = ContentContainer::new();
    cc.id = Some("contentcontainer0".into());
    cc.contents.push(reference(&pdf));
    let signed = fetch_response(
        vec![cc.clone()],
        Vec::new(),
        &manager_key,
        &manager_cert,
        &[],
        &[&pdf],
    );
    write_fixture("signed_attachment_response.osci", &signed);

    // 4. the same part encrypted on its own, its key in a container that is
    //    encrypted for the reader. Java must open both.
    let reader_slot = CertSlot {
        kind: osci_transport::role::RoleKind::Addressee,
        is_signature: false,
        cert_der: reader_cert.clone(),
    };
    let enc = encrypt_attachment(&pdf, SymmetricAlg::Aes128Cbc).unwrap();
    let mut cc = ContentContainer::new();
    cc.id = Some("contentcontainer0".into());
    cc.contents.push(reference(&pdf));
    cc.encrypted.push(enc.encrypted_data.clone());
    let xml = encode_content_container_bytes(&cc).unwrap();
    let sealed = encrypt_content_for_slot(
        &xml,
        &reader_cert,
        &reader_slot.id(),
        &EncryptOptions::default(),
    )
    .unwrap();
    let sealed_for_reader = sealed.clone();
    let bytes = fetch_response(
        Vec::new(),
        vec![sealed],
        &manager_key,
        &manager_cert,
        std::slice::from_ref(&reader_slot),
        &[&enc.part],
    );
    write_fixture("encrypted_attachment_response.osci", &bytes);

    // 5. the same container for a second reader, the author, in an
    //    EncryptedData of its own. Java must open the block for its reader and
    //    accept the other one.
    let author_slot = CertSlot {
        kind: osci_transport::role::RoleKind::Reader,
        is_signature: false,
        cert_der: manager_cert.clone(),
    };
    let for_author = encrypt_content_for_slot(
        &xml,
        &manager_cert,
        &author_slot.id(),
        &EncryptOptions::default(),
    )
    .unwrap();
    let bytes = fetch_response(
        Vec::new(),
        vec![sealed_for_reader, for_author],
        &manager_key,
        &manager_cert,
        &[reader_slot, author_slot],
        &[&enc.part],
    );
    write_fixture("two_readers_response.osci", &bytes);

    // 6. a response whose XML holds a line that looks like the MIME boundary.
    //    The message must pick another boundary, and Java must still read the
    //    text and verify the signature.
    let mut msg = response();
    msg.feedback[0].text = "vorher\r\n--MIME_boundary\r\nContent-ID: <x>\r\n\r\nnachher".into();
    let opts = ResponseOptions {
        signer: Some((manager_key.clone(), manager_cert.clone())),
        encrypt_for: None,
    };
    let bytes = respond_with(&msg, &opts).unwrap();
    write_fixture("boundary_in_xml_response.osci", &bytes);
}

fn reference(att: &Attachment) -> Content {
    Content {
        id: att.content_id.clone(),
        kind: ContentKind::Reference {
            href: format!("cid:{}", att.content_id.as_deref().unwrap()),
        },
    }
}

fn fetch_response(
    containers: Vec<ContentContainer>,
    encrypted: Vec<Vec<u8>>,
    key_pem: &[u8],
    cert_der: &[u8],
    extra_certs: &[CertSlot],
    attachments: &[&Attachment],
) -> Vec<u8> {
    let out = response();
    let mut control_block = out.control_block;
    control_block.challenge = Some("challenge".into());
    let mut env = ResponseEnvelope::new(MessageType::ResponseToFetchDelivery, control_block);
    env.feedback = out.feedback;
    env.content_package = Some(ContentPackage {
        containers,
        encrypted,
    });
    let parts: Vec<Attachment> = attachments.iter().map(|a| (*a).clone()).collect();
    let signed = sign_envelope_with_attachments(
        &env.encode().unwrap(),
        Role::Supplier,
        key_pem,
        cert_der,
        extra_certs,
        SignatureAlg::RsaSha256,
        &parts,
    )
    .unwrap();
    wrap_envelope_with(&signed, &parts).unwrap()
}
