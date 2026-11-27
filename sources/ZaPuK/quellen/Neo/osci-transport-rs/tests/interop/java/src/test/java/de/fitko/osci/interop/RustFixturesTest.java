// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.osci.interop;

import de.osci.osci12.common.CommonFactory;
import de.osci.osci12.common.DialogHandler;
import de.osci.osci12.messagetypes.FetchDelivery;
import de.osci.osci12.messagetypes.ResponseToFetchDelivery;
import de.osci.osci12.messagetypes.ResponseToStoreDelivery;
import de.osci.osci12.messageparts.Attachment;
import de.osci.osci12.messageparts.ContentContainer;
import de.osci.osci12.roles.Reader;
import de.osci.osci12.messagetypes.StoreDelivery;
import de.osci.osci12.roles.Addressee;
import de.osci.osci12.roles.Intermed;
import de.osci.osci12.roles.Originator;
import org.junit.jupiter.api.Test;

import java.net.URI;
import java.security.cert.X509Certificate;

import java.security.KeyFactory;
import java.security.spec.PKCS8EncodedKeySpec;
import java.util.Base64;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Feeds osci-transport-server response fixtures to an official Java osci-bibliothek
 * client via a {@link StubTransport} loopback, so a genuine Java parse
 * (MIME, signature) checks Rust's output.
 *
 * <p>Rust-produced <em>requests</em> aren't testable this way:
 * the official Java osci-bibliothek's only public parse entry point,
 * {@code PassiveRecipientParser}, is scoped to the two request types an
 * Intermediär forwards to a passive recipient (AcceptDelivery/
 * ProcessDelivery); its own request-signing path
 * ({@code OSCIRequest.transmit}) has no public hook to intercept the bytes
 * before they're sent.
 *
 * <p>Attachments are covered through a FetchDelivery loopback: the Supplier
 * signature Rust writes holds each part's digest, and an attachment Rust
 * encrypted with its own key is opened through the container encrypted for
 * the reader. The fixtures are kept small, since the real library's own C14N
 * canonicalizer fails on a response of ~9KB or more (a raw Xerces
 * SAXParseException deep inside the base64 CipherValue), independent of
 * anything Rust controls.
 */
class RustFixturesTest {

    private static final String SUPPLIER_CERT = "test_osci-manager_cypher.cer";
    private static final String ORIGINATOR_CERT = "Originator01.cer";

    private static DialogHandler dialogHandler(byte[] fixture, boolean checkSignatures) throws Exception {
        X509Certificate originatorCert = Keys.certificate(ORIGINATOR_CERT);
        X509Certificate supplierCert = Keys.certificate(SUPPLIER_CERT);

        Originator originator = new Originator(originatorCert, originatorCert);
        Intermed intermed = new Intermed(supplierCert, supplierCert, new URI("https://example.invalid/intermed"));

        DialogHandler dh = new DialogHandler(originator, intermed, new StubTransport(fixture));
        dh.setCreateSignatures(false);
        dh.setCheckSignatures(checkSignatures);
        dh.setEncryption(false);
        CommonFactory.setDisableControlBlockCheck(dh, true);
        // The dialog's own ControlBlock is normally populated by a prior
        // InitDialog round trip; the loopback skips that, so seed it enough
        // to pass compose()'s presence checks (the values themselves are
        // ignored, since the ControlBlock check is disabled above).
        seedControlBlock(dh);
        return dh;
    }

    private static void seedControlBlock(DialogHandler dh) {
        dh.getControlblock().setChallenge("challenge");
        dh.getControlblock().setResponse("response");
        dh.getControlblock().setConversationID("conversation");
        dh.getControlblock().setSequenceNumber(0);
    }

    private static ResponseToStoreDelivery sendStoreDelivery(String fixtureName, boolean checkSignatures) throws Exception {
        DialogHandler dh = dialogHandler(Keys.read(Keys.fixture(fixtureName)), checkSignatures);
        Addressee addressee = new Addressee(Keys.certificate(SUPPLIER_CERT), Keys.certificate(SUPPLIER_CERT));
        StoreDelivery req = new StoreDelivery(dh, addressee, "testmsg");
        return req.send();
    }

    @Test
    void unsignedResponseParses() throws Exception {
        ResponseToStoreDelivery resp = sendStoreDelivery("unsigned_response.osci", false);

        String[][] feedback = resp.getFeedback();
        assertEquals(1, feedback.length);
        assertEquals("9010101", feedback[0][1]);
    }

    @Test
    void signedResponseVerifies() throws Exception {
        ResponseToStoreDelivery resp = sendStoreDelivery("signed_response.osci", true);

        String[][] feedback = resp.getFeedback();
        assertEquals(1, feedback.length);
        assertEquals("9010101", feedback[0][1]);
    }

    private static final byte[] PDF = "%PDF-1.7\r\n\u0000\u0001\u00ff binary body"
        .getBytes(java.nio.charset.StandardCharsets.ISO_8859_1);

    private static ResponseToFetchDelivery fetch(String fixtureName) throws Exception {
        DialogHandler dh = dialogHandler(Keys.read(Keys.fixture(fixtureName)), true);
        FetchDelivery req = new FetchDelivery(dh);
        // the constructor clears what dialogHandler() seeded
        seedControlBlock(dh);
        return req.send();
    }

    // The attachment's digest sits in the Supplier signature Rust wrote; the
    // Java library recomputes it over the part and refuses on a mismatch.
    @Test
    void signedAttachmentVerifies() throws Exception {
        ResponseToFetchDelivery resp = fetch("signed_attachment_response.osci");

        ContentContainer[] containers = resp.getContentContainer();
        assertEquals(1, containers.length);
        Attachment[] attachments = containers[0].getAttachments();
        assertEquals(1, attachments.length);
        assertEquals("Klage.pdf", attachments[0].getRefID());
        assertEquals("application/pdf", attachments[0].getContentType());
        assertArrayEquals(PDF, attachments[0].getStream().readAllBytes());
    }

    // Rust encrypts the part with its own key and seals the key in a container
    // for the reader; Java opens the container and decrypts the part.
    @Test
    void encryptedAttachmentDecrypts() throws Exception {
        ResponseToFetchDelivery resp = fetch("encrypted_attachment_response.osci");
        assertEquals(1, resp.getEncryptedData().length);

        String pem = new String(Keys.read(Keys.keyFile("Reader01.key.pem")));
        byte[] der = Base64.getMimeDecoder().decode(pem
            .replaceAll("-----[A-Z ]+-----", ""));
        var key = KeyFactory.getInstance("RSA").generatePrivate(new PKCS8EncodedKeySpec(der));
        Reader reader = new Reader(new PemDecrypter(Keys.certificate("Reader01.cer"), key));

        ContentContainer opened = resp.getEncryptedData()[0].decrypt(reader);

        Attachment[] attachments = opened.getAttachments();
        assertEquals(1, attachments.length);
        assertEquals("Klage.pdf", attachments[0].getRefID());
        assertArrayEquals(PDF, attachments[0].getStream().readAllBytes());
    }

    // The same container sealed for the reader and for a second reader, each in
    // an EncryptedData of its own. Java opens the block for its reader.
    @Test
    void twoReadersEachHaveABlock() throws Exception {
        ResponseToFetchDelivery resp = fetch("two_readers_response.osci");
        assertEquals(2, resp.getEncryptedData().length);

        String pem = new String(Keys.read(Keys.keyFile("Reader01.key.pem")));
        byte[] der = Base64.getMimeDecoder().decode(pem
            .replaceAll("-----[A-Z ]+-----", ""));
        var key = KeyFactory.getInstance("RSA").generatePrivate(new PKCS8EncodedKeySpec(der));
        Reader reader = new Reader(new PemDecrypter(Keys.certificate("Reader01.cer"), key));

        ContentContainer opened = resp.getEncryptedData()[0].decrypt(reader);

        assertArrayEquals(PDF, opened.getAttachments()[0].getStream().readAllBytes());
    }

    // XML that holds a "--MIME_boundary" line: Rust writes the message with
    // another boundary, so the part is not cut and the signature holds.
    @Test
    void xmlThatLooksLikeTheBoundaryIsNotCut() throws Exception {
        ResponseToStoreDelivery resp = sendStoreDelivery("boundary_in_xml_response.osci", true);

        String[][] feedback = resp.getFeedback();
        assertEquals(1, feedback.length);
        assertEquals("9010101", feedback[0][1]);
        assertEquals(true, feedback[0][2].contains("--MIME_boundary"));
    }
}
