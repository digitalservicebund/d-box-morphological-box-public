// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import {
  MembershipChange,
  MessageType,
  MsgLikeKind,
  ProfileDetails,
  TimelineItemContent,
  VirtualTimelineItem,
} from '@matrix-g2x/neo-web-sdk';
import { TimelineItem, TimelineItemKind } from '@matrix-g2x/neo-web-sdk/react';
import { useTranslation } from 'react-i18next';
import './EventTile.scss';

function mxcToUrl(mxcUrl: string, homeserver: string, size = 48): string {
  // Normalize homeserver (remove trailing slash)
  const base = homeserver.replace(/\/$/, '');

  // Only accept the correct MXC scheme `mxc://` and extract the server and media id
  if (!mxcUrl.startsWith('mxc://')) {
    // invalid MXC form - throw so callers can handle the error explicitly
    throw new Error(`Invalid MXC url: ${mxcUrl}`);
  }
  const stripped = mxcUrl.replace(/^mxc:\/\//, '');
  // stripped is now <server>/<mediaId>
  const [serverFromMxc, ...mediaParts] = stripped.split('/');
  const mediaId = mediaParts.join('/');

  // Build the thumbnail url
  const url = new URL(
    `/_matrix/media/v3/thumbnail/${encodeURIComponent(serverFromMxc)}/${encodeURIComponent(mediaId)}`,
    base,
  );
  url.search = new URLSearchParams({
    width: String(size),
    height: String(size),
  }).toString();
  return url.toString();
}

type EventTileProps = {
  item: TimelineItem<TimelineItemKind>;
  homeserver?: string;
};

export default function EventTile({ item, homeserver }: EventTileProps) {
  const { t } = useTranslation();
  const showAvatar = !item.continuation;

  if (item.kind === TimelineItemKind.Spinner) {
    return <div>{t('eventTile.loading', 'Loading...')}</div>;
  }
  if (item.kind === TimelineItemKind.Virtual) {
    if (VirtualTimelineItem.DateDivider.instanceOf(item.item)) {
      return (
        <div
          style={{
            textAlign: 'center',
            margin: '10px 0',
            color: '#666',
          }}
        >
          <hr
            style={{
              border: 'none',
              borderTop: '1px solid #ccc',
              margin: '10px 0',
            }}
          />
          <span
            style={{
              backgroundColor: '#f0f0f0',
              padding: '0 10px',
              position: 'relative',
              top: '-12px',
            }}
          >
            {new Date(Number(item.item.inner.ts)).toLocaleDateString()}
          </span>
        </div>
      );
    }
    if (VirtualTimelineItem.ReadMarker.instanceOf(item.item)) {
      return (
        <div
          style={{
            textAlign: 'center',
            margin: '10px 0',
            color: '#666',
          }}
        >
          <hr
            style={{
              border: 'none',
              borderTop: '1px solid #ccc',
              margin: '10px 0',
            }}
          />
          <span
            style={{
              backgroundColor: '#f0f0f0',
              padding: '0 10px',
              position: 'relative',
              top: '-12px',
            }}
          >
            {t('eventTile.readMarker', 'Read')}
          </span>
        </div>
      );
    }
    if (VirtualTimelineItem.TimelineStart.instanceOf(item.item)) {
      return (
        <div
          style={{
            textAlign: 'center',
            margin: '10px 0',
            color: '#666',
          }}
        >
          <hr
            style={{
              border: 'none',
              borderTop: '1px solid #ccc',
              margin: '10px 0',
            }}
          />
          <span
            style={{
              backgroundColor: '#f0f0f0',
              padding: '0 10px',
              position: 'relative',
              top: '-12px',
            }}
          >
            {t('eventTile.timelineStart', 'Start of the timeline')}
          </span>
        </div>
      );
    }
    return (
      <div>
        {/*@ts-expect-error - tag is not defined but exists*/}
        {t('eventTile.unknownVirtual', 'Unknown virtual item')} {item.item.tag}
      </div>
    );
  }

  if (item.kind === TimelineItemKind.Event) {
    const event = item as TimelineItem<typeof TimelineItemKind.Event>;
    const senderProfile: Partial<{
      displayName?: string;
      displayNameAmbiguous?: boolean;
      avatarUrl?: string;
    }> = ProfileDetails.Ready.instanceOf(event.item.senderProfile)
      ? event.item.senderProfile.inner
      : {};
    const cardClass = 'event-tile-card event-tile-card--spaced';
    // Prepare display name
    const displayName = senderProfile.displayName || event.item.sender;
    // Prepare avatar URL if available
    let avatarNode: React.ReactNode | undefined;
    if (showAvatar && senderProfile.avatarUrl && homeserver) {
      const avatarUrl = mxcToUrl(senderProfile.avatarUrl, homeserver, 40);
      avatarNode = (
        <img
          src={avatarUrl}
          alt={displayName || 'Avatar'}
          style={{ width: 40, height: 40, borderRadius: '50%' }}
        />
      );
    }

    if (TimelineItemContent.MsgLike.instanceOf(event.item.content)) {
      const message = event.item.content.inner.content;

      if (MsgLikeKind.Redacted.instanceOf(message.kind)) {
        return (
          // @ts-expect-error - We are transitioning this code still
          <div className={cardClass} avatar={avatarNode} title={displayName}>
            <div style={{ fontStyle: 'italic', color: '#888' }}>
              {t('eventTile.redacted', 'Message has been redacted')}
            </div>
          </div>
        );
      } else if (MsgLikeKind.UnableToDecrypt.instanceOf(message.kind)) {
        return (
          // @ts-expect-error - We are transitioning this code still
          <div className={cardClass} avatar={avatarNode} title={displayName}>
            <div style={{ fontStyle: 'italic', color: '#888' }}>
              {t('eventTile.unableToDecrypt', 'Unable to decrypt message')}
            </div>
          </div>
        );
      } else if (MsgLikeKind.Message.instanceOf(message.kind)) {
        if (
          MessageType.Text.instanceOf(message.kind.inner.content.msgType) ||
          MessageType.Emote.instanceOf(message.kind.inner.content.msgType)
        ) {
          return (
            // @ts-expect-error - We are transitioning this code still
            <div avatar={avatarNode} className={cardClass} title={displayName}>
              <div style={{ whiteSpace: 'pre-wrap' }} className="kern-body">
                {message.kind.inner.content.body}
              </div>
            </div>
          );
        } else {
          return (
            // @ts-expect-error - We are transitioning this code still
            <div className={cardClass} avatar={avatarNode} title={displayName}>
              <div style={{ fontStyle: 'italic', color: '#888' }}>
                {t('eventTile.unsupportedType', 'Unsupported message type')}:{' '}
                {message.kind.inner.content.msgType.tag}
              </div>
            </div>
          );
        }
      } else {
        return (
          // @ts-expect-error - We are transitioning this code still
          <div className={cardClass} avatar={avatarNode}>
            {displayName && (
              <div style={{ fontWeight: 'bold', marginBottom: 4 }}>
                {displayName}
              </div>
            )}
            {t('eventTile.unknownMessageType', 'Unknown message type')}:{' '}
            {message.kind.tag}
          </div>
        );
      }
    } else if (
      TimelineItemContent.CallInvite.instanceOf(event.item.content) ||
      TimelineItemContent.RtcNotification.instanceOf(event.item.content) ||
      TimelineItemContent.State.instanceOf(event.item.content) ||
      TimelineItemContent.FailedToParseState.instanceOf(event.item.content) ||
      TimelineItemContent.FailedToParseMessageLike.instanceOf(
        event.item.content,
      )
    ) {
      // We dont show these to the user in this application ever
      return null;
    } else if (
      TimelineItemContent.ProfileChange.instanceOf(event.item.content)
    ) {
      // TODO: Do we want to show this to the user?
      return (
        // @ts-expect-error - We are transitioning this code still
        <div className={cardClass} avatar={avatarNode}>
          <div style={{ fontStyle: 'italic', color: '#888' }}>
            {t('eventTile.profileChange', 'Profile updated')}
          </div>
        </div>
      );
    } else if (
      TimelineItemContent.RoomMembership.instanceOf(event.item.content)
    ) {
      const membership = event.item.content.inner.change;
      let membershipText: string;
      if (membership === MembershipChange.Joined) {
        membershipText = t('eventTile.joined', '{{name}} joined the room', {
          name: displayName,
        });
      } else if (membership === MembershipChange.Left) {
        membershipText = t('eventTile.left', '{{name}} left the room', {
          name: displayName,
        });
      } else if (membership === MembershipChange.Banned) {
        membershipText = t(
          'eventTile.banned',
          '{{name}} was banned from the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.Invited) {
        membershipText = t(
          'eventTile.invited',
          '{{name}} was invited to the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.InvitationAccepted) {
        membershipText = t(
          'eventTile.invitationAccepted',
          '{{name}} accepted the invite to the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.InvitationRejected) {
        membershipText = t(
          'eventTile.invitationRejected',
          '{{name}} rejected the invite to the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.InvitationRevoked) {
        membershipText = t(
          'eventTile.invitationRevoked',
          '{{name}} revoked the invite to the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.Kicked) {
        membershipText = t(
          'eventTile.kicked',
          '{{name}} was removed from the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.KickedAndBanned) {
        membershipText = t(
          'eventTile.kickedAndBanned',
          '{{name}} was removed and banned from the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.Error) {
        membershipText = t(
          'eventTile.error',
          'Error processing membership change for {{name}}',
          { name: displayName },
        );
      } else if (membership === MembershipChange.Knocked) {
        membershipText = t(
          'eventTile.knocked',
          '{{name}} knocked on the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.KnockAccepted) {
        membershipText = t(
          'eventTile.knockAccepted',
          '{{name}} accepted the knock',
          { name: displayName },
        );
      } else if (membership === MembershipChange.KnockRetracted) {
        membershipText = t(
          'eventTile.knockRetracted',
          '{{name}} revoked the knock to the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.KnockDenied) {
        membershipText = t(
          'eventTile.knockDenied',
          '{{name}} rejected the knock to the room',
          { name: displayName },
        );
      } else if (membership === MembershipChange.Unbanned) {
        membershipText = t('eventTile.unbanned', '{{name}} has been unbanned', {
          name: displayName,
        });
      } else {
        membershipText =
          t(
            'eventTile.unknownMembershipChange',
            '{{name}} has an unknown membership change',
            { name: displayName },
          ) + ` (${membership})`;
      }
      return (
        // @ts-expect-error - We are transitioning this code still
        <div className={cardClass} avatar={avatarNode}>
          <div style={{ fontStyle: 'italic', color: '#888' }}>
            {membershipText}
          </div>
        </div>
      );
    } else {
      return (
        // @ts-expect-error - We are transitioning this code still
        <div className={cardClass} avatar={avatarNode}>
          {displayName && (
            <div style={{ fontWeight: 'bold', marginBottom: 4 }}>
              {displayName}
            </div>
          )}
          {t('eventTile.unknownEventContentType', 'Unknown event content type')}
        </div>
      );
    }
  }
  return (
    <div>
      {t('eventTile.unknownTimelineItemType', 'Unknown timeline item type')}
    </div>
  );
}
