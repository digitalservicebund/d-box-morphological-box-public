// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.api;

import javax.annotation.Nonnull;

import java.util.List;

import dev.fitko.neo.matrix.sdk.NeoContent;
import lombok.AccessLevel;
import lombok.Builder;

@Builder(builderClassName = "Builder", access = AccessLevel.PUBLIC)
public class SendAnswerRequest {
    public final @Nonnull String caseId;
    public final @Nonnull String dialogId;
    public final @Nonnull List<NeoContent> content;
}
