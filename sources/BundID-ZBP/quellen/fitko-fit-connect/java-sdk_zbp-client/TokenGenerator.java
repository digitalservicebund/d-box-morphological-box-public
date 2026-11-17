package dev.fitko.fitconnect.zbp.internal;

import com.nimbusds.jose.JOSEException;
import com.nimbusds.jose.JOSEObjectType;
import com.nimbusds.jose.JWSAlgorithm;
import com.nimbusds.jose.JWSHeader;
import com.nimbusds.jose.crypto.RSASSASigner;
import com.nimbusds.jose.jwk.RSAKey;
import com.nimbusds.jwt.JWTClaimsSet;
import com.nimbusds.jwt.SignedJWT;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.Date;
import java.util.List;

public final class TokenGenerator {

  private TokenGenerator() {}

  public static final int VALIDITY_IN_MINUTES = 30;
  public static final List<String> DEFAULT_ROLES = List.of("THIRD_PARTY");

  /**
   * Create a JWT token for access to the ZBP.
   *
   * @param signingKey the private key the token is signed with
   * @param signer name of the signer the token is issued for
   * @return JWT token as string
   * @throws RuntimeException if there was error signing the token
   */
  public static String buildToken(final RSAKey signingKey, final String signer) {

    final JWSHeader jwsHeader =
        new JWSHeader.Builder(JWSAlgorithm.RS512).type(JOSEObjectType.JWT).build();

    final Instant issueTime = Instant.now();

    final JWTClaimsSet claimsSet =
        new JWTClaimsSet.Builder()
            .issueTime(Date.from(issueTime))
            .expirationTime(Date.from(issueTime.plus(VALIDITY_IN_MINUTES, ChronoUnit.MINUTES)))
            .claim("signer", signer)
            .claim("roles", DEFAULT_ROLES)
            .build();

    final SignedJWT signedJWT = new SignedJWT(jwsHeader, claimsSet);
    try {
      final RSASSASigner rsassaSigner = new RSASSASigner(signingKey);
      signedJWT.sign(rsassaSigner);
      return signedJWT.serialize();
    } catch (JOSEException e) {
      throw new RuntimeException(e);
    }
  }
}
