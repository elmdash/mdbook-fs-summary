# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - Coming Soon

This is a bigger release aiming to encompass more of the default `SUMMARY.md` behaviors provided by mdbook. 

> ⚠️ Breaks links hardcoded to `00.html` to prefer `index.html` by default.

### Added

* [#1](https://github.com/elmdash/mdbook-fs-summary/issues/1) Convert `00.md` to `index.md` so that URLs show up as `index.html`.
* [#2](https://github.com/elmdash/mdbook-fs-summary/issues/2) Provide support for chapter separators using files that end in `__`
* [#3](https://github.com/elmdash/mdbook-fs-summary/issues/3) Provide support for prefix/suffix chapters using `00` and `ZZ` filename prefixes
* [#4](https://github.com/elmdash/mdbook-fs-summary/issues/4) Provide support for part titles with filenames ending in `#`
* [#5](https://github.com/elmdash/mdbook-fs-summary/issues/5) Provide support for draft chapters

### Changed

* No-op refactor to use mdbook's `Book` structures rather than just the `Summary` structures
* Ignore filename extensions when looking for suffix indicators

## [0.1.2] - 2021-12-29

### Fixes

* Directories without any markdown files are intentionally skipped, but that was not the case for directories with _only_ a `00.md` file. That's fixed now.

## [0.1.1] - 2021-12-28

### Added

* Initial implementation
* CHANGELOG & release workflow
* Basic README usage and installation

<!-- next-url -->
[Unreleased]: https://github.com/elmdash/mdbook-fs-summary/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/elmdash/mdbook-fs-summary/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Downstream/downsync/compare/v0.1.0...v0.1.1
