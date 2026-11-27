// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.osci.interop;

import de.osci.osci12.extinterfaces.TransportI;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.URI;

/**
 * A {@link TransportI} that discards whatever the real library sends and
 * hands back a fixed byte array as the "response". Lets a real
 * the official Java osci-bibliothek client parse a Rust-produced response fixture through its
 * normal receive path (signature check, transport decryption, content
 * decryption) without an actual network round trip.
 */
class StubTransport implements TransportI {
    private final byte[] response;

    StubTransport(byte[] response) {
        this.response = response;
    }

    @Override public String getVersion() { return "1"; }
    @Override public String getVendor() { return "FITKO osci-transport interop harness"; }
    @Override public TransportI newInstance() { return this; }
    @Override public InputStream getResponseStream() { return new ByteArrayInputStream(response); }
    @Override public boolean isOnline(URI uri) { return true; }
    @Override public long getContentLength() { return response.length; }

    @Override
    public OutputStream getConnection(URI uri, long length) throws IOException {
        return new ByteArrayOutputStream();
    }
}
