# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Blanket `impl`s to `Client`
- `StreamingClient` for streaming connections
- `UploadFileClient` for uploading files

### Changed

- Update the documentation
- Take `io::Read` instead of file path in file uploads
- Adjust lifetime specification of request methods
