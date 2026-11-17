package dev.fitko.fitconnect.zbp;

import dev.fitko.fitconnect.zbp.internal.ZBPEnvelopes;
import dev.fitko.fitconnect.zbp.model.AuthorKeyPair;
import dev.fitko.fitconnect.zbp.model.CreateMessage;
import dev.fitko.fitconnect.zbp.model.CreateState;
import java.util.function.Supplier;

/** Signs ZBP messages and status updates for sending through the FIT-Connect ZBP adapter. */
public final class ZBPEnvelopeBuilder {

  private ZBPEnvelopeBuilder() {}

  /**
   * Signs a message with the author's key pair and wraps it in a ZBP envelope.
   *
   * @param message the message for the citizen's mailbox
   * @param authorKeyPair the author's private key and certificate
   * @return the envelope as JSON, to be sent as submission data to the ZBP adapter
   * @throws ZBPException if the message cannot be signed
   */
  public static byte[] fromAuthorPayload(
      final CreateMessage message, final AuthorKeyPair authorKeyPair) {
    return sign(
        () -> ZBPEnvelopes.fromAuthorPayload(message, authorKeyPair), "Could not sign ZBP message");
  }

  /**
   * Signs a status update with the author's key pair and wraps it in a ZBP envelope.
   *
   * @param state the status update for the application
   * @param authorKeyPair the author's private key and certificate
   * @return the envelope as JSON, to be sent as submission data to the ZBP adapter
   * @throws ZBPException if the status update cannot be signed
   */
  public static byte[] fromAuthorPayload(
      final CreateState state, final AuthorKeyPair authorKeyPair) {
    return sign(
        () -> ZBPEnvelopes.fromAuthorPayload(state, authorKeyPair),
        "Could not sign ZBP status update");
  }

  private static byte[] sign(final Supplier<byte[]> signing, final String errorMessage) {
    try {
      return signing.get();
    } catch (final RuntimeException e) {
      throw new ZBPException(errorMessage, e);
    }
  }
}
