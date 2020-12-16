# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Handle ping/pong with server

### Changed

- Follow changes of `Client` in `misskey-core`
  - Adjust lifetime specification of request methods
- Implement `StreamingClient`
- Improve API of `ReconnectCondition` and `ReconnectConfig`
- Improve API of `WebSocketClient`
- Improve API of `WebSocketClientBuilder`
