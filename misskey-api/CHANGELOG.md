# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support for Misskey v12.65.4 ~ v12.65.7
   - endpoint `get-online-users-count`
- Support for Misskey v12.66.0
   - endpoint `server-info`
- Support for Misskey v12.67.0 ~ v12.68.0
   - endpoint `ping`
   - endpoint `i/registry/*`
   - registry-related model types, such as `RegistryKey`, `RegistryValueType`, and `RegistryScope`
   - `RegistryUpdated` variant to `MainStreamEvent`
- Support for Misskey v12.69.0
- Support for Misskey v12.70.0
- Support for Misskey v12.71.0 ~ v12.74.1
- Support for Misskey v12.75.0 ~ v12.76.1
- `muted_notification_types` user setting which is available since v12.48.0
- Page related endpoints
   - endpoint `pages/*`
   - endpoint `i/pages`
   - endpoint `i/page_likes`
- Support for Misskey v12.77.0
- Support for Misskey v12.77.1 ~ v12.78.0
   - endpoint `notifications/read`
- Support for Misskey v12.79.0 ~ v12.79.1
   - endpoint `gallery/*`
   - endpoint `i/gallery/*`
   - endpoint `users/gallery/posts`
- Support for Misskey v12.79.2 ~ v12.79.3
   - endpoint `gallery/posts/delete`
   - endpoint `gallery/posts/update`
- Partial support for Misskey v12.80.0 ~ v12.80.3
   - endpoint `admin/ad/*`
- Support for Misskey v12.81.0 ~ 12.81.2
   - endpoint `admin/get-index-stats`
- Support for Misskey v12.82.0 ~ v12.87.0
- Partial support for Misskey v12.88.0
- Support for Misskey v12.89.0
- Support for Misskey v12.89.1 ~ v12.90.1
- Support for Misskey v12.91.0
   - endpoint `admin/accounts/delete`
- Support for Misskey v12.92.0
   - endpoint `email-address/available`
   - endpoint `users/groups/leave`
- Partial support for Misskey v12.93.0 ~ v12.94.1
   - endpoint `users/reactions`
- Support for Misskey v12.95.0
   - endpoint `notes/thread-muting/*`
- Support for Misskey v12.96.0 ~ v12.97.1
- Support for Misskey v12.98.0
   - endpoint `following/invalidate`
- Partial support for Misskey v12.99.0 ~ v12.101.1
- Support for Misskey v12.102.0 ~ v12.103.1
   - endpoint `admin/emoji/*`
- Support for Misskey v12.104.0
   - endpoint `charts/ap-request`
- Support for Misskey v12.105.0
- Support for Misskey v12.106.0 ~ v12.106.3
- Support for Misskey v12.107.0
- Support for Misskey v12.108.0 ~ v12.108.1
- Support for Misskey v12.109.0 ~ v12.110.1
   - endpoint `admin/meta`
- Support for Misskey v12.111.0 ~ v12.111.1
- Partial support for Misskey v12.112.0 ~ v12.112.2
   - endpoint `admin/delete-account`
   - endpoint `admin/get-user-ips`
   - endpoint `admin/update-user-note`
   - endpoint `admin/drive-capacity-override`
   - endpoint `clips/remove-note`
- Support for Misskey v12.112.3 ~ v12.119.2
- Support for Misskey v13.0.0
   - endpoint `admin/roles/*`
   - endpoint `flash/*`
   - endpoint `emojis`
   - endpoint `invite`
   - endpoint `retention`
   - endpoint `charts/user/pv`
- Support for Misskey v13.1.0
   - endpoint `i/claim-achievement`
   - endpoint `users/achievements`
- Support for Misskey v13.1.1 ~ v13.2.2
- Support for Misskey v13.2.3
   - broadcast event `emojiDeleted`
   - broadcast event `emojiUpdated`
- Support for Misskey v13.2.4 ~ v13.2.6
- Support for Misskey v13.3.0 ~ v13.3.1
- Support for Misskey v13.3.2 ~ v13.3.4
- Support for Misskey v13.4.0 ~ v13.6.1
- Support for Misskey v13.7.0 ~ v13.7.5
   - endpoint `admin/roles/users`
   - endpoint `roles/*`
- Support for Misskey v13.8.0 ~ v13.8.1
- Support for Misskey v13.9.0 ~ v13.9.2
- Support for Misskey v13.10.0 ~ v13.10.2
   - endpoint `renote-mute/*`
   - endpoint `clips/favorite`
   - endpoint `clips/my-favorites`
   - endpoint `clips/unfavorite`
   - endpoint `emoji`
- Support for Misskey v13.10.3
- Partial support for v13.11.0 ~ v13.11.1
   - endpoint `channels/favorite`
   - endpoint `channels/my-favorites`
   - endpoint `channels/unfavorite`
- Support for Misskey v13.11.2
   - endpoint `channels/search`
- Support for Misskey v13.11.3
   - endpoint `roles/note`
   - channel `roleTimeline`
- Support for Misskey v13.12.0
   - endpoint `admin/emoji/set-license-bulk`
   - endpoint `users/update-memo`

### Changed

- Moved `streaming::emoji::EmojiAddedEvent` to `streaming::broadcast::emoji_added::EmojiAddedEvent`

### Deprecated
### Removed

- `ClientSettingUpdated` variant from `MainStreamEvent`
   - For Misskey v12.67.0 ~ v12.68.0
- Latest version flag from being enabled as default
- endpoint `users`
   - For Misskey v12.88.0
- endpoint `users/recommendation`
   - For Misskey v12.88.0 ~
- endpoint `admin/logs` and `admin/delete-logs`
   - For Misskey v12.93.0 ~
- endpoint `admin/emoji/remove`
   - For Misskey v12.102.0 ~
- endpoint `charts/network`
   - For Misskey v12.104.0 ~
- endpoint `stats`
   - For Misskey v12.106.0 ~ v12.106.3
- endpoint `email-address/available`
   - For Misskey v12.108.0 ~ v12.108.1
- endpoint `admin/invite`, `admin/moderators/*`, `admin/silence_user`, `admin/unsilence_user`, `admin/vaccum`, `notes/watching/*`
   - For Misskey v13.0.0 ~
- endpoint `messaging/*`, `i/read_all_messaging_messages`, `i/user_group_invites`, `users/groups`
   - For Misskey v13.7.0 ~
- channel `messaging`, `messagingIndex`
   - For Misskey v13.7.0 ~

### Fixed

- Fix `admin/show-user` response

### Security

## [0.2.0] - 2020-12-17

### Added

- Trait for entity model types: `Entity` and `EntityRef`
- Trait for pagination: `PaginationRequest` and `OffsetPaginationRequest`
- Unified representation of IDs: `Id<T>`
- Support for ID generation methods
- `Query` type for DNF queries
- Missing `Default` implementations on some request types
- Support for Misskey v12.48.0 ~ v12.48.3
- Support for Misskey v12.49.0 ~ v12.50.0
- Support for Misskey v12.51.0 ~ v12.54.0
- Support for Misskey v12.55.0 ~ v12.56.0
- Support for Misskey v12.57.0 ~ v12.57.4
- Support for Misskey v12.58.0 ~ v12.59.0
- Support for Misskey v12.60.0 ~ v12.60.1
- Support for Misskey v12.61.0 ~ v12.61.1
- Support for Misskey v12.62.0 ~ v12.62.1
- Support for Misskey v12.62.2
- Support for Misskey v12.63.0
- `notes/reactions` endpoint
- `NoteReaction` entity
- `PaginationRequest::set_limit` and `OffsetPaginationRequest::set_limit` to give access to the `limit` field

### Changed

- Update some words in the description
- Use `Mime` from `mime` crate for file types
- Use uninhabited type `NoOutgoing` instead of `()` for `ConnectChannelRequest::Outgoing` of channels that do not send out messages
- Forbid to construct error types outside the module
- Rename `model::user::UserSort` to `UserSortKey`
- Expose the implementation of ID

### Removed

- `model::messaging::UserGroup`, which is duplicate of `model::user_group::UserGroup`

### Fixed

- Fix some model and request definitions
