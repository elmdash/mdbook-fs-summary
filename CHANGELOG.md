# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - Coming Soon

## [0.2.1] - 2022-01-04

### Changed

* Add clean paths configuration option to allow disabling the path changes which currently breaks mdbook's partials handling.

## [0.2.0] - 2021-12-31

This is a bigger release aiming to encompass more of the default `SUMMARY.md` behaviors provided by mdbook and establish more conventions.

### Added

* [#2](https://github.com/elmdash/mdbook-fs-summary/issues/2) Provide support for chapter separators using files that end in `__`
* [#3](https://github.com/elmdash/mdbook-fs-summary/issues/3) Provide support for prefix/suffix chapters using `00` and `ZZ` filename prefixes
* [#4](https://github.com/elmdash/mdbook-fs-summary/issues/4) Provide support for part titles with filenames ending in `#`
* [#5](https://github.com/elmdash/mdbook-fs-summary/issues/5) Provide support for draft chapters

### Changed

* [#1](https://github.com/elmdash/mdbook-fs-summary/issues/1) Convert `00.md` to `index.md` so that URLs show up as `index.html`.

  > ⚠️ Breaks links hardcoded to `00.html` to prefer `index.html` by default.

* [#6](https://github.com/elmdash/mdbook-fs-summary/issues/6) Strip number prefixes from final URLs

  > ⚠️ Also breaks links hardcoded with numbering prefixes in favor of stripping prefixes by default: `02_team/01_directory/06_birthdays.md` becomes`team/directory/birthdays.html`.
  >
  > This also seems to break the partial includes which relies on the wrong path value.

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
[Unreleased]: https://github.com/elmdash/mdbook-fs-summary/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/elmdash/mdbook-fs-summary/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/elmdash/mdbook-fs-summary/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/elmdash/mdbook-fs-summary/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Downstream/downsync/compare/v0.1.0...v0.1.1
