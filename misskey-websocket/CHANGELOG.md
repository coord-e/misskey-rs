# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Accept additional headers for `WebSocketClient`

### Changed

- Use tokio 1.0 and async-tungstenite 0.13
  - Drop feature flags for tokio 0.2 and async-tungstenite 0.9

### Deprecated
### Removed

- implementation of `Client` to `WebSocketClient`
  - For Misskey v12.111.0 ~

### Fixed
### Security

## [0.2.0] - 2020-12-17

### Fixed

- Handle ping/pong with server

### Changed

- Follow changes of `Client` in `misskey-core`
  - Adjust lifetime specification of request methods
- Implement `StreamingClient`
- Improve API of `ReconnectCondition` and `ReconnectConfig`
- Improve API of `WebSocketClient`
- Improve API of `WebSocketClientBuilder`
- Use tokio 0.3 and async-tungstenite 0.10
