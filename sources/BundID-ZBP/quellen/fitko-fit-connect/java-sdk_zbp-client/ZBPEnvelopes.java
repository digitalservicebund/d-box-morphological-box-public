package dev.fitko.fitconnect.zbp.internal;

import com.nimbusds.jose.JOSEException;
import com.nimbusds.jose.jwk.RSAKey;
import dev.fitko.fitconnect.core.crypto.util.CertUtils;
import dev.fitko.fitconnect.zbp.model.AuthorKeyPair;
import dev.fitko.fitconnect.zbp.model.CreateMessage;
import dev.fitko.fitconnect.zbp.model.CreateState;
import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.security.Signature;
import java.security.SignatureException;
import java.util.Base64;
import tools.jackson.core.JacksonException;
import tools.jackson.databind.ObjectMapper;

/** Signs a {@link CreateMessage} or {@link CreateState} and wraps it in a {@link ZBPEnvelope}. */
public final class ZBPEnvelopes {

  private static final ObjectMapper MAPPER = ObjectMapperProvider.createObjectMapper();

  private ZBPEnvelopes() {}

  /**
   * Write payload to a {@link ZBPEnvelope} as byte[] that is signed with the authors private key.
   *
   * @param payload the @{@link CreateMessage} or {@link CreateState} payload that is written to the
   *     {@link ZBPEnvelope}
   * @param authorKeyPair pair of author client certificate and private key the token and content
   *     signature is created from
   * @return signed {@link ZBPEnvelope} as byte array
   */
  public static byte[] fromAuthorPayload(final Object payload, final AuthorKeyPair authorKeyPair) {
    final String authorCertificate = authorKeyPair.authorCertificateAsPem();
    final RSAKey authorPrivateKey = authorKeyPair.authorPrivateKey();
    final String signer = CertUtils.getSubjectCNFromCertificate(authorCertificate);
    final String token = TokenGenerator.buildToken(authorPrivateKey, signer);
    final ZBPEnvelope envelope = buildEnvelope(payload, authorPrivateKey, authorCertificate, token);
    return serializeToByteArray(envelope);
  }

  /**
   * Write payload to a {@link ZBPEnvelope} that is signed with the senders private key.
   *
   * @param payload the @{@link CreateMessage} or {@link CreateState} payload that is written to the
   *     {@link ZBPEnvelope}
   * @param senderPrivateKey sender private key
   * @param authorCertificate client certificate of the author
   * @param authorToken token of the author
   * @return signed {@link ZBPEnvelope} as byte array
   */
  public static ZBPEnvelope fromSenderPayload(
      final Object payload,
      final RSAKey senderPrivateKey,
      final String authorCertificate,
      final String authorToken) {
    return buildEnvelope(payload, senderPrivateKey, authorCertificate, authorToken);
  }

  private static ZBPEnvelope buildEnvelope(
      final Object payload,
      final RSAKey privateKey,
      final String authorCertificate,
      final String authorToken) {
    return ZBPEnvelope.builder()
        .content(serializeToJson(payload))
        .sha512sum(signPayload(payload, privateKey))
        .authorCertificate(authorCertificate)
        .authorToken(authorToken)
        .build();
  }

  private static String signPayload(final Object input, final RSAKey key) {
    try {
      final Signature signature = Signature.getInstance("SHA512withRSA");
      signature.initSign(key.toPrivateKey());
      signature.update(MAPPER.writeValueAsBytes(input));
      final byte[] encoded = Base64.getEncoder().encode(signature.sign());
      return new String(encoded, StandardCharsets.UTF_8);
    } catch (NoSuchAlgorithmException
        | SignatureException
        | InvalidKeyException
        | JOSEException
        | JacksonException e) {
      throw new RuntimeException(e);
    }
  }

  private static byte[] serializeToByteArray(final Object obj) {
    try {
      return MAPPER.writeValueAsBytes(obj);
    } catch (JacksonException e) {
      throw new RuntimeException(e);
    }
  }

  private static String serializeToJson(final Object payload) {
    return new String(serializeToByteArray(payload), StandardCharsets.UTF_8);
  }
}
