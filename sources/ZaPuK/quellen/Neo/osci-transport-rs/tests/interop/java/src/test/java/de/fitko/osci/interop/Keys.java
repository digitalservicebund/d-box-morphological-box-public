// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.osci.interop;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.security.GeneralSecurityException;
import java.security.cert.CertificateFactory;
import java.security.cert.X509Certificate;

/** Loads the PEM/DER key material shared with the Rust test fixtures. */
final class Keys {

    private Keys() {
    }

    // tests/interop/{keys,generated}, relative to this Gradle project root.
    private static final Path INTEROP_ROOT = Paths.get("..").toAbsolutePath().normalize();

    static Path keyFile(String name) {
        return INTEROP_ROOT.resolve("keys").resolve(name);
    }

    static Path fixture(String name) {
        return INTEROP_ROOT.resolve("generated").resolve(name);
    }

    static byte[] read(Path path) throws IOException {
        return Files.readAllBytes(path);
    }

    static InputStream stream(Path path) throws IOException {
        return new ByteArrayInputStream(read(path));
    }

    static X509Certificate certificate(String derFileName) throws IOException, GeneralSecurityException {
        CertificateFactory cf = CertificateFactory.getInstance("X.509");
        try (InputStream in = stream(keyFile(derFileName))) {
            return (X509Certificate) cf.generateCertificate(in);
        }
    }
}
