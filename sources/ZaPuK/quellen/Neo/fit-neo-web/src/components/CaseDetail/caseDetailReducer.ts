// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { InquiryFormState, ModalState } from './caseDetailHelpers';

export interface CaseDetailState {
  formState: InquiryFormState;
  submitting: boolean;
  modalState: ModalState;
  isAddingAmendment: boolean;
  amendmentText: string;
  submittingAmendment: boolean;
}

export type CaseDetailAction =
  | { type: 'SET_FORM'; value: InquiryFormState }
  | { type: 'SET_MODAL'; state: ModalState }
  | { type: 'CLOSE_MODAL' }
  | { type: 'SUBMIT_START' }
  | { type: 'SUBMIT_DONE' }
  | { type: 'SUBMIT_SUCCESS' }
  | { type: 'START_AMENDMENT' }
  | { type: 'CANCEL_AMENDMENT' }
  | { type: 'SET_AMENDMENT_TEXT'; text: string }
  | { type: 'AMENDMENT_SUBMIT_START' }
  | { type: 'AMENDMENT_SUBMIT_DONE' }
  | { type: 'AMENDMENT_SUCCESS' };

export const initialCaseDetailState: CaseDetailState = {
  formState: {
    texts: new Map(),
    polls: new Map(),
    pollCustom: new Map(),
    files: new Map(),
    fileDeclarations: new Map(),
    fileComments: new Map(),
  },
  submitting: false,
  modalState: { kind: 'idle' },
  isAddingAmendment: false,
  amendmentText: '',
  submittingAmendment: false,
};

export function caseDetailReducer(
  state: CaseDetailState,
  action: CaseDetailAction,
): CaseDetailState {
  switch (action.type) {
    case 'SET_FORM':
      return { ...state, formState: action.value };
    case 'SET_MODAL':
      return { ...state, modalState: action.state };
    case 'CLOSE_MODAL':
      return { ...state, modalState: { kind: 'idle' } };
    case 'SUBMIT_START':
      return { ...state, submitting: true };
    case 'SUBMIT_DONE':
      return { ...state, submitting: false };
    case 'SUBMIT_SUCCESS':
      return { ...state, submitting: false, modalState: { kind: 'success' } };
    case 'START_AMENDMENT':
      return { ...state, isAddingAmendment: true };
    case 'CANCEL_AMENDMENT':
      return { ...state, isAddingAmendment: false, amendmentText: '' };
    case 'SET_AMENDMENT_TEXT':
      return { ...state, amendmentText: action.text };
    case 'AMENDMENT_SUBMIT_START':
      return { ...state, submittingAmendment: true };
    case 'AMENDMENT_SUBMIT_DONE':
      return { ...state, submittingAmendment: false };
    case 'AMENDMENT_SUCCESS':
      return {
        ...state,
        submittingAmendment: false,
        isAddingAmendment: false,
        amendmentText: '',
        modalState: { kind: 'success' },
      };
  }
}
