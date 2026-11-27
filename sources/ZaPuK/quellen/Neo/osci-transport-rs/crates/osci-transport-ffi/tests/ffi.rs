// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::mime::main_xml_part;
use osci_transport::parts::{Content, ContentContainer, ContentKind};
use osci_transport::wire::encode_content_container_bytes;
use osci_transport_ffi::{
    AttachmentDto, CertOwnerDto, CertSlotDto, ContentKindDto, MessageKind, ReplyContentDto,
    build_reply_payload, cert_slot_id, classify_message, decrypt_content_container,
    decrypt_content_container_with_attachments, decrypt_message_content, decrypt_transport_message,
    encrypt_container_with_attachments, encrypt_content_container_for_slot, envelope_certificates,
    generate_cipher_key, message_attachments, sign_request, verify_attachment_digests,
    wrap_message_with_attachments,
};

fn interop(p: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/interop")
        .join(p)
}

#[test]
fn classifies_a_plain_message() {
    let xml = main_xml_part(&fs::read(interop("corpus/FetchProcessCard.osci")).unwrap()).unwrap();
    match classify_message(xml).unwrap() {
        MessageKind::Message { message_type } => assert_eq!(message_type, "FetchProcessCard"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn decrypts_transport_then_content() {
    let manager_pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let manager_cer = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let raw = fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap();

    let inner = decrypt_transport_message(raw, manager_pem, manager_cer).unwrap();
    let xml = main_xml_part(&inner).unwrap();
    assert!(matches!(
        classify_message(xml).unwrap(),
        MessageKind::Message { .. }
    ));

    // content container from a plain StoreDelivery, for the reader key
    let reader_pem = fs::read_to_string(interop("keys/Reader01.key.pem")).unwrap();
    let sd = fs::read(interop("corpus/StoreDelivery.osci")).unwrap();
    let env = osci_transport::envelope::parse(&main_xml_part(&sd).unwrap()).unwrap();
    let ed = env
        .content_package
        .unwrap()
        .encrypted
        .into_iter()
        .next()
        .unwrap();

    let cc = decrypt_content_container(
        ed,
        reader_pem,
        "reader0_cipher_a5ed201258618adb054546e503e685a32ab73968".into(),
    )
    .unwrap();
    assert!(
        cc.contents
            .iter()
            .any(|c| matches!(c.kind, ContentKindDto::Inline))
    );
}

#[test]
fn content_encrypted_for_a_slot_decrypts_by_the_slot_id() {
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let cer = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let mut cc = osci_transport::parts::ContentContainer::new();
    cc.id = Some("cc0".into());
    let xml = osci_transport::wire::encode_content_container_bytes(&cc).unwrap();

    let ed = encrypt_content_container_for_slot(xml, cer, "addressee_cipher".into()).unwrap();

    assert!(String::from_utf8_lossy(&ed).contains(r##"URI="#addressee_cipher""##));
    let back = decrypt_content_container(ed, pem, "addressee_cipher".into()).unwrap();
    assert_eq!(back.id.as_deref(), Some("cc0"));
}

fn pdf() -> AttachmentDto {
    AttachmentDto {
        content_id: Some("Klage.pdf".into()),
        content_type: Some("application/pdf".into()),
        data: b"%PDF-1.7\r\n\x00\x01\xff body".to_vec(),
    }
}

// What the web client does to send: seal the container with its attachments,
// put the parts on the wire. The reader reverses it.
#[test]
fn attachments_round_trip_through_the_ffi() {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let mut cc = ContentContainer::new();
    cc.contents.push(Content {
        id: Some("Klage.pdf".into()),
        kind: ContentKind::Reference {
            href: "cid:Klage.pdf".into(),
        },
    });
    let container = encode_content_container_bytes(&cc).unwrap();

    let sealed = encrypt_container_with_attachments(
        container,
        vec![pdf()],
        cert,
        Some("addressee_cipher".into()),
    )
    .unwrap();
    assert_ne!(sealed.attachments[0].data, pdf().data);
    assert_eq!(sealed.attachments[0].content_id, pdf().content_id);

    let wire =
        wrap_message_with_attachments(b"<envelope/>".to_vec(), sealed.attachments.clone()).unwrap();
    let received = message_attachments(wire).unwrap();
    assert_eq!(received.len(), 1);

    let opened = decrypt_content_container_with_attachments(
        sealed.encrypted_data_xml,
        pem,
        "addressee_cipher".into(),
        received,
    )
    .unwrap();
    assert_eq!(opened.attachments[0].data, pdf().data);
    assert_eq!(opened.container.contents.len(), 1);
}

#[test]
fn a_part_the_container_has_no_key_for_is_an_error() {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let container = encode_content_container_bytes(&ContentContainer::new()).unwrap();
    let sealed = encrypt_container_with_attachments(container, Vec::new(), cert, None).unwrap();

    let err = decrypt_content_container_with_attachments(
        sealed.encrypted_data_xml,
        pem,
        "addressee_cipher".into(),
        vec![pdf()],
    );
    assert!(err.is_err());
}

// The Java library's own signed StoreDelivery with an attachment.
#[test]
fn attachment_digests_are_checked_against_the_signature() {
    let raw = fs::read(interop("corpus/StoreDelivery.osci")).unwrap();
    let xml = main_xml_part(&raw).unwrap();
    let parts = message_attachments(raw).unwrap();
    assert_eq!(parts.len(), 1);

    // the signer comes back, so the caller can check who it is
    let signer = verify_attachment_digests(xml.clone(), parts.clone()).unwrap();
    assert!(!signer.is_empty());

    let mut swapped = parts;
    swapped[0].data.push(b'!');
    assert!(verify_attachment_digests(xml, swapped).is_err());
}

// The whole send path of the web client: seal the container for the reader,
// sign the request over the encrypted part, put it on the wire. The receiver
// checks the part against the signature and opens it.
#[test]
fn a_signed_request_with_an_encrypted_attachment_checks_out() {
    use osci_transport::dialog::ControlBlock;
    use osci_transport::wire::{Request, RequestEnvelope};

    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let reader = CertSlotDto {
        owner: CertOwnerDto::Addressee,
        is_signature: false,
        certificate_der: cert.clone(),
    };
    let slot = cert_slot_id(reader.clone());
    assert_eq!(slot, "addressee_cipher");

    let mut cc = ContentContainer::new();
    cc.contents.push(Content {
        id: Some("Klage.pdf".into()),
        kind: ContentKind::Reference {
            href: "cid:Klage.pdf".into(),
        },
    });
    let sealed = encrypt_container_with_attachments(
        encode_content_container_bytes(&cc).unwrap(),
        vec![pdf()],
        cert.clone(),
        Some(slot),
    )
    .unwrap();

    let envelope = RequestEnvelope::new(
        Request::GetMessageId,
        ControlBlock {
            id: Some("controlblock".into()),
            sequence_number: Some(0),
            ..Default::default()
        },
    )
    .encode()
    .unwrap();
    let signed = sign_request(
        envelope,
        pem,
        cert,
        vec![reader],
        sealed.attachments.clone(),
    )
    .unwrap();
    let wire = wrap_message_with_attachments(signed, sealed.attachments).unwrap();

    let xml = main_xml_part(&wire).unwrap();
    let parts = message_attachments(wire).unwrap();
    verify_attachment_digests(xml.clone(), parts.clone()).unwrap();

    let mut swapped = parts;
    swapped[0].data[0] ^= 1;
    assert!(verify_attachment_digests(xml, swapped).is_err());
}

// The Java library's own request: the originator's certificates are in its header.
#[test]
fn certificates_of_a_received_envelope() {
    let xml = main_xml_part(&fs::read(interop("corpus/StoreDelivery.osci")).unwrap()).unwrap();

    let certs = envelope_certificates(xml).unwrap();

    let cipher = certs
        .iter()
        .find(|c| matches!(c.owner, CertOwnerDto::Originator) && !c.is_signature)
        .expect("originator cipher certificate");
    assert!(
        cipher
            .id
            .as_deref()
            .unwrap()
            .starts_with("originator_cipher")
    );
    assert_eq!(cipher.certificate_der[0], 0x30, "DER sequence");
    assert!(
        certs
            .iter()
            .any(|c| matches!(c.owner, CertOwnerDto::Originator) && c.is_signature)
    );
}

// The Java library's StoreDelivery: its content is encrypted for the reader, whose
// slot is found by the key, and its attachment is sent plain.
#[test]
fn a_java_message_is_read_in_one_call() {
    let raw = fs::read(interop("corpus/StoreDelivery.osci")).unwrap();
    let pem = fs::read_to_string(interop("keys/Reader01.key.pem")).unwrap();

    let content = decrypt_message_content(raw.clone(), pem).unwrap();

    assert!(
        content
            .containers
            .iter()
            .flat_map(|c| &c.contents)
            .any(|c| matches!(c.kind, ContentKindDto::Inline))
    );
    let plain = message_attachments(raw).unwrap();
    assert_eq!(content.attachments.len(), plain.len());
    assert_eq!(content.attachments[0].data, plain[0].data);
}

#[test]
fn a_message_for_another_key_is_refused() {
    let raw = fs::read(interop("corpus/StoreDelivery.osci")).unwrap();
    let other = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();

    assert!(decrypt_message_content(raw, other).is_err());
}

// What we send (encrypted attachment in an encrypted container) is read back in
// one call from the whole message.
#[test]
fn an_encrypted_attachment_is_read_in_one_call() {
    use osci_transport::dialog::ControlBlock;
    use osci_transport::message::MessageType;
    use osci_transport::parts::ContentPackage;
    use osci_transport::wire::ResponseEnvelope;

    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let reader = CertSlotDto {
        owner: CertOwnerDto::Addressee,
        is_signature: false,
        certificate_der: cert.clone(),
    };
    let mut cc = ContentContainer::new();
    cc.contents.push(Content {
        id: Some("Klage.pdf".into()),
        kind: ContentKind::Reference {
            href: "cid:Klage.pdf".into(),
        },
    });
    let sealed = encrypt_container_with_attachments(
        encode_content_container_bytes(&cc).unwrap(),
        vec![pdf()],
        cert.clone(),
        Some(cert_slot_id(reader.clone())),
    )
    .unwrap();
    let mut env = ResponseEnvelope::new(
        MessageType::ResponseToFetchDelivery,
        ControlBlock {
            id: Some("controlblock".into()),
            sequence_number: Some(0),
            ..Default::default()
        },
    );
    env.content_package = Some(ContentPackage {
        containers: Vec::new(),
        encrypted: vec![sealed.encrypted_data_xml],
    });
    let signed = sign_request(
        env.encode().unwrap(),
        pem.clone(),
        cert,
        vec![reader],
        sealed.attachments.clone(),
    )
    .unwrap();
    let wire = wrap_message_with_attachments(signed, sealed.attachments).unwrap();

    let content = decrypt_message_content(wire, pem).unwrap();

    assert_eq!(content.attachments[0].data, pdf().data);
    assert_eq!(content.attachments[0].content_id, pdf().content_id);
    assert_eq!(content.containers.len(), 1);
}

fn reply() -> (Vec<u8>, String) {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let payload = build_reply_payload(
        vec![ReplyContentDto {
            id: Some("xjustiz".into()),
            bytes: b"<reply>hello back</reply>".to_vec(),
        }],
        vec![pdf()],
        cert,
        None,
    )
    .unwrap();
    (payload, pem)
}

// What the bridge accepts: a ContentPackage with the container as top-level
// EncryptedData, for slot addressee_cipher, and the encrypted part beside it.
#[test]
fn a_reply_payload_opens_for_the_sender() {
    let (payload, pem) = reply();

    let package =
        osci_transport::wire::parse_content_package(&main_xml_part(&payload).unwrap()).unwrap();
    assert!(package.containers.is_empty());
    assert_eq!(package.encrypted.len(), 1);
    let parts = message_attachments(payload).unwrap();
    assert_eq!(parts.len(), 1);
    assert_ne!(parts[0].data, pdf().data, "the part is sent encrypted");

    let opened = decrypt_content_container_with_attachments(
        package.encrypted[0].clone(),
        pem,
        "addressee_cipher".into(),
        parts,
    )
    .unwrap();
    assert_eq!(opened.attachments[0].data, pdf().data);
    assert_eq!(opened.attachments[0].content_type, pdf().content_type);
    let contents = &opened.container.contents;
    assert!(contents.iter().any(
        |c| matches!(c.kind, ContentKindDto::Inline) && c.bytes == b"<reply>hello back</reply>"
    ));
    assert!(
        contents
            .iter()
            .any(|c| matches!(c.kind, ContentKindDto::Reference)
                && c.href.as_deref() == Some("cid:Klage.pdf"))
    );
}

#[test]
fn a_reply_attachment_needs_a_content_id() {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let mut part = pdf();
    part.content_id = None;

    assert!(build_reply_payload(Vec::new(), vec![part], cert, None).is_err());
}

#[test]
fn a_reply_refuses_an_id_used_twice() {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();

    // two attachments of the same name
    assert!(build_reply_payload(Vec::new(), vec![pdf(), pdf()], cert.clone(), None).is_err());
    // an inline content with the id of an attachment
    let content = ReplyContentDto {
        id: pdf().content_id,
        bytes: b"<x/>".to_vec(),
    };
    assert!(build_reply_payload(vec![content], vec![pdf()], cert, None).is_err());
}

// The ids the encoder makes up count as well: the container's own, and
// content<N> for a content without an id.
#[test]
fn a_reply_refuses_an_id_the_encoder_uses() {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let with_id = |id: &str| ReplyContentDto {
        id: Some(id.into()),
        bytes: b"<x/>".to_vec(),
    };
    assert!(
        build_reply_payload(
            vec![with_id("contentcontainer0")],
            vec![],
            cert.clone(),
            None
        )
        .is_err()
    );
    let anonymous = ReplyContentDto {
        id: None,
        bytes: b"<y/>".to_vec(),
    };
    assert!(build_reply_payload(vec![anonymous, with_id("content0")], vec![], cert, None).is_err());
}

// The container is sealed as the caller wrote it: ids and XMLContent stay.
#[test]
fn sealing_a_container_keeps_its_content() {
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let container = br#"<osci:ContentContainer xmlns:osci="http://www.osci.de/2002/04/osci" Id="c"><osci:Content><osci:Base64Content Id="foo">YWJj</osci:Base64Content></osci:Content><osci:Content Id="w"><osci:XMLContent Id="y"><a/></osci:XMLContent></osci:Content><osci:Content Id="Klage.pdf" href="cid:Klage.pdf"></osci:Content></osci:ContentContainer>"#;
    let sealed = encrypt_container_with_attachments(
        container.to_vec(),
        vec![pdf()],
        cert,
        Some("addressee_cipher".into()),
    )
    .unwrap();
    let opened = decrypt_content_container_with_attachments(
        sealed.encrypted_data_xml,
        pem,
        "addressee_cipher".into(),
        sealed.attachments,
    )
    .unwrap();
    assert_eq!(opened.attachments[0].data, pdf().data);
    let contents = &opened.container.contents;
    assert_eq!(contents.len(), 3);
    assert_eq!(contents[0].bytes, b"abc");
    // the XMLContent was not turned into base64 text
    assert!(contents[1].bytes.starts_with(b"<osci:XMLContent"));
    assert_eq!(contents[2].id.as_deref(), Some("Klage.pdf"));
}

// A strict XML parser needs the prefixes of the package root declared.
#[test]
fn the_reply_package_declares_its_namespaces() {
    let (payload, _) = reply();

    let xml = String::from_utf8(main_xml_part(&payload).unwrap()).unwrap();

    assert!(xml.starts_with("<osci:ContentPackage xmlns:"), "{xml}");
}

#[test]
fn a_generated_key_is_pkcs8_pem_and_comes_with_a_request() {
    let k = generate_cipher_key(2048, "CN=Court Mailbox".into()).unwrap();

    assert!(k.private_key_pem.starts_with("-----BEGIN PRIVATE KEY-----"));
    assert_eq!(k.csr_der[0], 0x30, "a DER sequence");
    assert!(generate_cipher_key(1024, "CN=x".into()).is_err());
}

// A reply for the court and for its author: each reads it with their own key,
// the slot is found by the certificate in the header.
#[test]
fn a_reply_is_read_by_the_court_and_by_its_author() {
    use osci_transport::dialog::ControlBlock;
    use osci_transport::message::MessageType;
    use osci_transport::wire::ResponseEnvelope;

    let court_cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let court_pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let author_cert = fs::read(interop("keys/Reader01.cer")).unwrap();
    let author_pem = fs::read_to_string(interop("keys/Reader01.key.pem")).unwrap();
    let payload = build_reply_payload(
        vec![ReplyContentDto {
            id: Some("xjustiz".into()),
            bytes: b"<reply>hello back</reply>".to_vec(),
        }],
        vec![pdf()],
        court_cert.clone(),
        Some(author_cert.clone()),
    )
    .unwrap();
    let package =
        osci_transport::wire::parse_content_package(&main_xml_part(&payload).unwrap()).unwrap();
    assert_eq!(package.encrypted.len(), 2);
    let parts = message_attachments(payload).unwrap();
    assert_eq!(parts.len(), 1, "the part is encrypted once");

    // the header the bridge writes: both readers, with the Ids the blocks name
    let slots = |owner, cert: &Vec<u8>| CertSlotDto {
        owner,
        is_signature: false,
        certificate_der: cert.clone(),
    };
    let readers = vec![
        slots(CertOwnerDto::Addressee, &court_cert),
        slots(CertOwnerDto::Reader, &author_cert),
    ];
    assert_eq!(cert_slot_id(readers[1].clone()), "otherreader_cipher");
    let mut env = ResponseEnvelope::new(
        MessageType::ResponseToFetchDelivery,
        ControlBlock {
            id: Some("controlblock".into()),
            sequence_number: Some(0),
            ..Default::default()
        },
    );
    env.content_package = Some(package);
    let signed = sign_request(
        env.encode().unwrap(),
        court_pem.clone(),
        court_cert,
        readers,
        parts.clone(),
    )
    .unwrap();
    let wire = wrap_message_with_attachments(signed, parts).unwrap();

    for pem in [court_pem, author_pem] {
        let content = decrypt_message_content(wire.clone(), pem).unwrap();
        assert_eq!(content.attachments[0].data, pdf().data);
        assert_eq!(content.containers.len(), 1);
        assert!(
            content.containers[0]
                .contents
                .iter()
                .any(|c| c.bytes == b"<reply>hello back</reply>")
        );
    }
}

#[test]
fn a_message_that_fails_to_read_names_the_step() {
    let pem = fs::read_to_string(interop("keys/Reader01.key.pem")).unwrap();

    let Err(e) = decrypt_message_content(b"<broken & xml".to_vec(), pem) else {
        panic!("a broken message was read");
    };

    assert!(e.to_string().contains("message"), "{e}");
}
