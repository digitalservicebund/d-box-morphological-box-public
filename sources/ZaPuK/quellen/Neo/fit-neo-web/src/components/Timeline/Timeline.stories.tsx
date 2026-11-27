// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';
import type { TimelineProps } from './Timeline';
import Timeline from './Timeline';

const meta = {
  title: 'Components/Navigation/Timeline',
  component: Timeline,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
  decorators: [
    (Story) => (
      <div style={{ width: '395px', padding: '0 16px' }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof Timeline>;
export default meta;
type Story = StoryObj<typeof meta>;

const sampleSections: TimelineProps['sections'] = [
  {
    id: 'sep-2025',
    label: 'September 2025',
    entries: [
      {
        id: 'entry-1',
        date: '15.09.2025',
        sender: 'Behörde Berlin',
        title: 'Bescheid',
        body: 'Anfrage erhalten',
        attachmentName: 'Anhang.pdf',
        extraCount: 2,
      },
      {
        id: 'entry-2',
        date: '20.09.2025',
        sender: 'Behörde Berlin',
        title: 'Rückfrage',
        body: 'Zusätzliche Unterlagen benötigt',
        attachmentName: 'Anhang.pdf',
        extraCount: 2,
      },
    ],
  },
  {
    id: 'oct-2025',
    label: 'Oktober 2025',
    entries: [
      {
        id: 'entry-3',
        date: '05.10.2025',
        sender: 'Bürger',
        title: 'Dokumente eingereicht',
        body: 'Personalausweis und Meldebescheinigung',
        attachmentName: 'Personalausweis.pdf',
        extraCount: 1,
      },
      {
        id: 'entry-4',
        date: '12.10.2025',
        sender: 'Behörde Berlin',
        title: 'Abschlussbescheid',
        body: 'Ihr Antrag wurde bewilligt',
      },
    ],
  },
];

const allStatesSections: TimelineProps['sections'] = [
  {
    id: 'states-2025',
    label: 'Oktober 2025',
    entries: [
      {
        id: 'state-default',
        date: '01.10.2025',
        sender: 'Behörde Berlin',
        title: 'Standard (gelesen)',
        body: 'Normaler gelesener Eintrag',
      },
      {
        id: 'state-new',
        date: '02.10.2025',
        sender: 'Behörde Berlin',
        title: 'Neu / Ungelesen',
        body: 'Ungelesene Nachricht mit blauem Punkt',
        isNew: true,
      },
      {
        id: 'state-warning',
        date: '03.10.2025',
        sender: 'Behörde Berlin',
        title: 'Warnung',
        body: 'Handlungsbedarf erforderlich',
        status: 'warning',
        attachmentName: 'Anhang.pdf',
      },
      {
        id: 'state-warning-new',
        date: '04.10.2025',
        sender: 'Behörde Berlin',
        title: 'Warnung (neu)',
        body: 'Neue Warnung, noch ungelesen',
        status: 'warning',
        isNew: true,
      },
      {
        id: 'state-success',
        date: '05.10.2025',
        sender: 'Behörde Berlin',
        title: 'Erfolg',
        body: 'Vorgang erfolgreich abgeschlossen',
        status: 'success',
      },
      {
        id: 'state-deadline',
        date: '06.10.2025',
        sender: 'Behörde Berlin',
        title: 'Fristablauf',
        body: 'Bitte reagieren Sie bis zum Fristdatum',
        status: 'warning',
        deadline: 'Frist: 31.10.2025',
      },
    ],
  },
];

export const Default: Story = {
  args: {
    sections: sampleSections,
  },
};

export const WithSelection: Story = {
  render: (args) => {
    const [selectedId, setSelectedId] = useState<string | undefined>('entry-1');
    return (
      <Timeline {...args} selectedId={selectedId} onSelect={setSelectedId} />
    );
  },
  args: {
    sections: sampleSections,
  },
};

export const SingleSection: Story = {
  args: {
    sections: [
      {
        id: 'sep-2025',
        label: 'September 2025',
        entries: [
          {
            id: 'entry-1',
            date: '15.09.2025',
            sender: 'Behörde Berlin',
            title: 'Bescheid',
            body: 'Anfrage erhalten',
            attachmentName: 'Anhang.pdf',
            extraCount: 2,
          },
        ],
      },
    ],
  },
};

export const NoAttachments: Story = {
  args: {
    sections: [
      {
        id: 'sep-2025',
        label: 'September 2025',
        entries: [
          {
            id: 'entry-1',
            date: '15.09.2025',
            sender: 'Behörde Berlin',
            title: 'Bescheid',
            body: 'Anfrage erhalten',
          },
        ],
      },
    ],
  },
};

export const Empty: Story = {
  args: {
    sections: [],
  },
};

export const CustomTitle: Story = {
  args: {
    title: 'Verlauf',
    sections: sampleSections,
  },
};

export const WithReply: Story = {
  args: {
    sections: [
      {
        id: 'sep-2025',
        label: 'September 2025',
        entries: [
          {
            id: 'entry-with-reply',
            date: '15.09.2025',
            sender: 'Behörde Berlin',
            title: 'Bescheid',
            body: 'Anfrage erhalten',
            status: 'success',
            attachmentName: 'Anhang.pdf',
            extraCount: 2,
            reply: {
              date: '10.09.2025',
              sender: 'Bürger',
              title: 'Antrag auf Ummeldung',
              body: 'Ich möchte meinen Wohnsitz ummelden.',
            },
          },
          {
            id: 'entry-reply-no-body',
            date: '20.09.2025',
            sender: 'Behörde Berlin',
            title: 'Rückfrage',
            body: 'Zusätzliche Unterlagen benötigt',
            reply: {
              date: '15.09.2025',
              sender: 'Behörde Berlin',
              title: 'Bescheid',
            },
          },
        ],
      },
    ],
  },
};

export const AllStates: Story = {
  args: {
    sections: allStatesSections,
  },
};

export const AllStatesWithSelection: Story = {
  render: (args) => {
    const [selectedId, setSelectedId] = useState<string | undefined>(
      'state-new',
    );
    return (
      <Timeline {...args} selectedId={selectedId} onSelect={setSelectedId} />
    );
  },
  args: {
    sections: allStatesSections,
  },
};
