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

### Changed
### Deprecated
### Removed

- `ClientSettingUpdated` variant from `MainStreamEvent`
   - For Misskey v12.67.0 ~ v12.68.0
- Latest version flag from being enabled as default

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
