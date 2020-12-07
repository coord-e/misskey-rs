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

### Changed

- Update some words in the description
- Use `Mime` from `mime` crate for file types
- Use uninhabited type `NoOutgoing` instead of `()` for `ConnectChannelRequest::Outgoing` of channels that do not send out messages

### Removed

- `model::messaging::UserGroup`, which is duplicate of `model::user_group::UserGroup`

### Fixed

- Fix some model and request definitions
