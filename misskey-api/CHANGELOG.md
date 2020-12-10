# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- Support for Misskey v12.55.0

### Changed

- Update some words in the description
- Use `Mime` from `mime` crate for file types
- Use uninhabited type `NoOutgoing` instead of `()` for `ConnectChannelRequest::Outgoing` of channels that do not send out messages
- Forbid to construct error types outside the module

### Removed

- `model::messaging::UserGroup`, which is duplicate of `model::user_group::UserGroup`

### Fixed

- Fix some model and request definitions
