package dev.fitko.fitconnect.zbp.internal;

import com.nimbusds.jose.jwk.RSAKey;
import dev.fitko.fitconnect.core.crypto.util.CertUtils;
import dev.fitko.fitconnect.core.http.api.HttpService;
import dev.fitko.fitconnect.core.http.api.RestApiException;
import dev.fitko.fitconnect.zbp.ZBPCertConfig;
import dev.fitko.fitconnect.zbp.ZBPEnvironment;
import dev.fitko.fitconnect.zbp.model.CreateMessageResponse;
import dev.fitko.fitconnect.zbp.model.ZBPApiAttachment;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import okhttp3.MediaType;
import okhttp3.MultipartBody;
import okhttp3.RequestBody;
import tools.jackson.core.JacksonException;
import tools.jackson.databind.ObjectMapper;

public class ZBPApiService {

  public static final String MAILBOX_MESSAGE_PATH = "/v6/mailbox/messages";
  public static final String CREATE_STATE_PATH = "/v6/mailbox/applications/states";
  public static final String ZBP_PEM_COOKIE_NAME = "AL_ENV_SSL_CLIENT_CERT";
  private static final ObjectMapper OBJECT_MAPPER = new ObjectMapper();

  private final HttpService httpClient;
  private final String baseUrl;
  private final ZBPEnvironment environment;
  private final ZBPCertConfig certConfig;

  public ZBPApiService(
      final HttpService httpClient,
      final ZBPEnvironment environment,
      final ZBPCertConfig certConfig) {
    this.httpClient = httpClient;
    this.baseUrl = environment.getBaseUrl();
    this.environment = environment;
    this.certConfig = certConfig;
  }

  /**
   * Insert a new message with optional attachments to the user mailbox.
   *
   * @param signingKey key to sign the access token
   * @param envelope the signed envelope that contains the message payload
   * @param attachments the attachments for the message, if empty no attachments will be sent
   * @return {@link CreateMessageResponse}
   * @throws RestApiException if there was a technical error sending the message
   */
  public CreateMessageResponse sendMessageToMailbox(
      final RSAKey signingKey,
      final String signer,
      final ZBPEnvelope envelope,
      final List<ZBPApiAttachment> attachments)
      throws RestApiException {
    final String url = String.format(baseUrl + MAILBOX_MESSAGE_PATH);
    try {
      final String token = TokenGenerator.buildToken(signingKey, signer);
      final Map<String, String> headers = buildHeaders(token, "multipart/form-data");
      final MultipartBody multipartBody = buildMultipartBody(envelope, attachments);
      return httpClient.put(url, headers, multipartBody, CreateMessageResponse.class).body();
    } catch (final Exception e) {
      throw new RestApiException("Could not send message to zbp mailbox", e);
    }
  }

  /**
   * Create a new state for a given application.
   *
   * @param signingKey key to sign the access token
   * @param createStateEnvelope the signed envelope that contains the signed state payload
   * @throws RestApiException if there was a technical error creating the state
   */
  public void createNewState(
      final RSAKey signingKey, final String signer, final ZBPEnvelope createStateEnvelope)
      throws RestApiException {
    final String url = String.format(baseUrl + CREATE_STATE_PATH);
    try {
      final String token = TokenGenerator.buildToken(signingKey, signer);
      final Map<String, String> headers = buildHeaders(token, "application/json");
      httpClient.post(url, headers, createStateEnvelope, Void.class);
    } catch (final Exception e) {
      throw new RestApiException("Could not create application state", e);
    }
  }

  private MultipartBody buildMultipartBody(
      final ZBPEnvelope messageEnvelope, final List<ZBPApiAttachment> attachments) {
    final var multipartBodyBuilder =
        new MultipartBody.Builder()
            .setType(MultipartBody.FORM)
            .addFormDataPart("json", writeToString(messageEnvelope));

    for (ZBPApiAttachment attachment : attachments) {
      final RequestBody requestBody =
          RequestBody.create(MediaType.parse(attachment.mimeType()), attachment.data());
      multipartBodyBuilder.addFormDataPart("files", attachment.filename(), requestBody);
    }

    return multipartBodyBuilder.build();
  }

  private Map<String, String> buildHeaders(final String bearerToken, final String contentType) {
    final HashMap<String, String> headers = new HashMap<>();
    headers.put("Authorization", "Bearer " + bearerToken);
    headers.put("Content-Type", contentType);

    if (environment.isSendCertAsCookie()) {
      headers.put(
          "Cookie",
          ZBP_PEM_COOKIE_NAME + "=" + CertUtils.removeMarker(certConfig.getClientCertificate()));
    }

    return headers;
  }

  private String writeToString(final ZBPEnvelope envelope) {
    try {
      return OBJECT_MAPPER.writeValueAsString(envelope);
    } catch (JacksonException e) {
      throw new RuntimeException(e);
    }
  }
}
