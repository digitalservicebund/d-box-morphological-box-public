// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.osci.interop;

import de.osci.osci12.extinterfaces.crypto.Decrypter;

import javax.crypto.Cipher;
import javax.crypto.spec.OAEPParameterSpec;
import javax.crypto.spec.PSource;
import java.security.PrivateKey;
import java.security.spec.MGF1ParameterSpec;
import java.security.cert.X509Certificate;

/** A {@link Decrypter} over a PKCS#8 PEM key from the shared test keys. */
final class PemDecrypter extends Decrypter {
    private final X509Certificate cert;
    private final PrivateKey key;

    PemDecrypter(X509Certificate cert, PrivateKey key) {
        this.cert = cert;
        this.key = key;
    }

    @Override public String getVersion() { return "1"; }
    @Override public String getVendor() { return "FITKO osci-transport interop harness"; }
    @Override public X509Certificate getCertificate() { return cert; }

    @Override
    public byte[] decrypt(byte[] data) {
        try {
            Cipher c = Cipher.getInstance("RSA/ECB/PKCS1Padding");
            c.init(Cipher.DECRYPT_MODE, key);
            return c.doFinal(data);
        } catch (Exception e) {
            throw new IllegalStateException(e);
        }
    }

    @Override
    public byte[] decrypt(byte[] data, String mgfAlgorithm, String digestAlgorithm) {
        try {
            Cipher c = Cipher.getInstance("RSA/ECB/OAEPPadding");
            c.init(Cipher.DECRYPT_MODE, key, new OAEPParameterSpec(
                hash(digestAlgorithm), "MGF1", new MGF1ParameterSpec(hash(mgfAlgorithm)),
                PSource.PSpecified.DEFAULT));
            return c.doFinal(data);
        } catch (Exception e) {
            throw new IllegalStateException(e);
        }
    }

    // an xmlenc digest URI or an MGF1 URI names its hash at the end
    private static String hash(String uri) {
        if (uri.endsWith("sha512")) return "SHA-512";
        if (uri.endsWith("sha384")) return "SHA-384";
        if (uri.endsWith("sha256")) return "SHA-256";
        if (uri.endsWith("sha1")) return "SHA-1";
        throw new IllegalArgumentException(uri);
    }
}
