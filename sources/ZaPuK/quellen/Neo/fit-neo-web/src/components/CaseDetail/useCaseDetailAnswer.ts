// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoDialog } from '@matrix-g2x/neo-web-sdk/web';
import { useReducer } from 'react';
import { useTranslation } from 'react-i18next';
import { POLL_CUSTOM_OPTION_ID } from './caseDetailHelpers';
import { caseDetailReducer, initialCaseDetailState } from './caseDetailReducer';

export function useCaseDetailAnswer({
  dialog,
  displayNames,
  onAnswer,
  onUploadMedia,
}: {
  dialog: NeoDialog;
  displayNames?: Record<string, string>;
  onAnswer?: (blocks: NeoContent[]) => Promise<void>;
  onUploadMedia?: (data: ArrayBuffer, mimeType: string) => Promise<string>;
}) {
  const { t } = useTranslation();
  const [
    {
      formState,
      submitting,
      modalState,
      isAddingAmendment,
      amendmentText,
      submittingAmendment,
    },
    dispatch,
  ] = useReducer(caseDetailReducer, initialCaseDetailState);

  function insertFormula() {
    const firstTextBlock = dialog.message.blocks.find((b) =>
      NeoContent.InquiryMessage.instanceOf(b.content),
    );

    if (!firstTextBlock) return;
    const ownSender = dialog.answers.at(-1)?.blocks[0]?.sender ?? '';
    const ownName = (ownSender && displayNames?.[ownSender]) ?? '';
    const formulaBase = t('caseDetail.formula', 'Mit freundlichen Grüßen');
    const formula = ownName ? `${formulaBase}\n${ownName}` : formulaBase;
    const current = formState.texts.get(firstTextBlock.id) ?? '';
    const next = current.trim() ? `${current}\n\n${formula}` : formula;
    const texts = new Map(formState.texts);
    texts.set(firstTextBlock.id, next);
    dispatch({ type: 'SET_FORM', value: { ...formState, texts } });
  }

  function handleSubmitClick() {
    const hasInquiryFile = dialog.message.blocks.some((b) =>
      NeoContent.InquiryFile.instanceOf(b.content),
    );

    if (hasInquiryFile) {
      const fileBlock = dialog.message.blocks.find((b) =>
        NeoContent.InquiryFile.instanceOf(b.content),
      );
      if (!fileBlock) return;

      const file = formState.files.get(fileBlock.id) ?? null;
      const isDeclaration =
        formState.fileDeclarations.get(fileBlock.id) ?? false;

      if (file) {
        dispatch({
          type: 'SET_MODAL',
          state: { kind: 'confirm-file', file, blockId: fileBlock.id },
        });
      } else if (isDeclaration) {
        const comment = formState.fileComments.get(fileBlock.id) ?? '';
        dispatch({
          type: 'SET_MODAL',
          state: {
            kind: 'confirm-declaration',
            blockId: fileBlock.id,
            text: t(
              'caseDetail.noFileDeclaration',
              'Ich habe die geforderte Anlage nicht fristgerecht vorliegend.',
            ),
            comment,
          },
        });
      }
      return;
    }

    // Build preview items for text/poll confirmation modal
    const previews: { answer: string; comment?: string }[] = [];
    for (const block of dialog.message.blocks) {
      if (NeoContent.InquiryMessage.instanceOf(block.content)) {
        const text = formState.texts.get(block.id)?.trim();
        if (text) previews.push({ answer: text });
      }
      if (NeoContent.InquiryPoll.instanceOf(block.content)) {
        const poll = block.content.inner.poll;
        const selections = formState.polls.get(block.id) ?? [];
        if (selections.length) {
          const customText = formState.pollCustom.get(block.id)?.trim();
          const selectionSet = new Set(selections);
          const hasCustom = selectionSet.has(POLL_CUSTOM_OPTION_ID);
          // Preserve poll.answers order rather than selection order
          const answerParts: string[] = [];
          for (const a of poll.answers) {
            if (!selectionSet.has(a.id)) continue;
            answerParts.push(
              a.id === POLL_CUSTOM_OPTION_ID
                ? t(
                    'caseDetail.pollCustomOption',
                    'Keine der genannten Antworten',
                  )
                : (a.text.plain ?? a.id),
            );
          }
          const answerLabel = answerParts.join(', ');
          previews.push({
            answer: answerLabel,
            comment: hasCustom && customText ? customText : undefined,
          });
        }
      }
    }

    if (previews.length > 0) {
      dispatch({
        type: 'SET_MODAL',
        state: { kind: 'confirm-answer', previews },
      });
      return;
    }

    submitAnswer();
  }

  function submitAnswer(): void {
    actuallySubmit().catch((error: unknown) => {
      console.error('Submit failed:', error);
      dispatch({ type: 'SUBMIT_DONE' });
    });
  }

  async function actuallySubmit() {
    if (!onAnswer) return;
    if (modalState.kind === 'confirm-amendment') {
      await submitAmendment(modalState.text);
      return;
    }
    const blocks: NeoContent[] = [];

    if (modalState.kind === 'confirm-file') {
      const { file, blockId } = modalState;
      if (onUploadMedia) {
        try {
          dispatch({ type: 'SUBMIT_START' });
          const data = await file.arrayBuffer();
          const mxcUri = await onUploadMedia(data, file.type);
          const inquiryBlock = dialog.message.blocks.find(
            (b) => b.id === blockId,
          );
          const inquiryTitle =
            inquiryBlock &&
            NeoContent.InquiryFile.instanceOf(inquiryBlock.content)
              ? inquiryBlock.content.inner.title?.plain
              : undefined;
          blocks.push(
            new NeoContent.File({
              file: {
                uri: mxcUri,
                name: file.name,
                mimetype: file.type,
                size: BigInt(file.size),
              },
              title: inquiryTitle
                ? { plain: inquiryTitle, html: undefined }
                : undefined,
              replyTo: blockId,
            }),
          );
        } catch (err) {
          console.error('Upload failed:', err);
          dispatch({ type: 'SUBMIT_DONE' });
          return;
        }
      }
    } else if (modalState.kind === 'confirm-declaration') {
      const { blockId, text, comment } = modalState;
      const fullText = comment.trim() ? `${text}\n\n${comment}` : text;
      blocks.push(
        new NeoContent.Message({
          text: { plain: fullText, html: undefined },
          replyTo: blockId,
        }),
      );
    } else {
      // Direct submit (InquiryMessage / InquiryPoll — no modal)
      for (const block of dialog.message.blocks) {
        if (NeoContent.InquiryMessage.instanceOf(block.content)) {
          const text = formState.texts.get(block.id) ?? '';
          if (text.trim()) {
            blocks.push(
              new NeoContent.Message({
                text: { plain: text, html: undefined },
                replyTo: block.id,
              }),
            );
          }
        } else if (NeoContent.InquiryPoll.instanceOf(block.content)) {
          const selections = formState.polls.get(block.id) ?? [];
          if (selections.length) {
            const customText = formState.pollCustom.get(block.id);
            blocks.push(
              new NeoContent.Poll({
                selections,
                custom: customText || undefined,
                replyTo: block.id,
              }),
            );
          }
        }
      }
    }

    if (!blocks.length) return;

    dispatch({ type: 'SUBMIT_START' });
    try {
      if (blocks.length) await onAnswer(blocks);
      dispatch({ type: 'SUBMIT_SUCCESS' });
    } catch (err) {
      console.error('Send failed:', err);
      dispatch({ type: 'SUBMIT_DONE' });
    }
  }

  const hasAnswer = dialog.message.blocks.some((block) => {
    const { id, content } = block;
    if (NeoContent.InquiryMessage.instanceOf(content)) {
      return !!formState.texts.get(id)?.trim();
    }
    if (NeoContent.InquiryPoll.instanceOf(content)) {
      return (formState.polls.get(id)?.length ?? 0) > 0;
    }
    if (NeoContent.InquiryFile.instanceOf(content)) {
      return (
        formState.files.get(id) != null ||
        formState.fileDeclarations.get(id) === true
      );
    }
    return false;
  });

  const hasIncompleteCustomPollAnswer = dialog.message.blocks.some(
    ({ id, content }) =>
      NeoContent.InquiryPoll.instanceOf(content) &&
      formState.polls.get(id)?.includes(POLL_CUSTOM_OPTION_ID) &&
      !formState.pollCustom.get(id)?.trim(),
  );

  const canSubmit =
    !submitting && !!onAnswer && hasAnswer && !hasIncompleteCustomPollAnswer;

  // TODO: derive from SDK once it exposes whether a dialog allows amendments
  const allowAmendment = dialog.message.blocks.some(({ content }) => {
    let plainTexts: (string | undefined)[] = [];
    if (NeoContent.Message.instanceOf(content)) {
      plainTexts = [content.inner.text.plain];
    } else if (NeoContent.InquiryMessage.instanceOf(content)) {
      plainTexts = [content.inner.title?.plain, content.inner.label?.plain];
    } else if (NeoContent.InquiryFile.instanceOf(content)) {
      plainTexts = [content.inner.title?.plain, content.inner.label?.plain];
    } else if (NeoContent.InquiryPoll.instanceOf(content)) {
      plainTexts = [
        content.inner.poll.question.plain,
        ...content.inner.poll.answers.map((answer) => answer.text.plain),
      ];
    }
    return plainTexts.some((text) => text?.includes('[amendments]'));
  });

  function handleSendAmendment() {
    if (!onAnswer || !amendmentText.trim() || submittingAmendment) return;
    dispatch({
      type: 'SET_MODAL',
      state: { kind: 'confirm-amendment', text: amendmentText.trim() },
    });
  }

  async function submitAmendment(text: string) {
    if (!onAnswer || submittingAmendment) return;
    dispatch({ type: 'AMENDMENT_SUBMIT_START' });
    try {
      await onAnswer([
        new NeoContent.Message({
          text: { plain: text, html: undefined },
          replyTo: undefined,
        }),
      ]);
      dispatch({ type: 'AMENDMENT_SUCCESS' });
    } catch (err) {
      console.error('Amendment failed:', err);
      dispatch({ type: 'AMENDMENT_SUBMIT_DONE' });
    }
  }

  return {
    formState,
    submitting,
    modalState,
    isAddingAmendment,
    amendmentText,
    submittingAmendment,
    dispatch,
    insertFormula,
    handleSubmitClick,
    submitAnswer,
    canSubmit,
    allowAmendment,
    handleSendAmendment,
  };
}
