# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Follow changes of `Client` in `misskey-core`
  - Adjust lifetime specification of request methods
- Implement `UploadFileClient`
  - Take `io::Read` instead of file path in file uploads
- Stop taking the token as `Option` in the constructor and provide a separate method.
- Improve API of `HttpClientBuilder`
