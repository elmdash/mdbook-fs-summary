# mdbook-fs-summary

[![crates.io](https://img.shields.io/crates/v/mdbook-fs-summary.svg)](https://crates.io/crates/mdbook-fs-summary)
[![LICENSE](https://img.shields.io/github/license/elmdash/mdbook-fs-summary.svg)](LICENSE)

A preprocessor that generates a summary from the file structure of your book rather than using an explicit `SUMMARY.md` file.

### Benefits

Frequently, you want the organization of your files to simply be mirrored in the `SUMMARY.md` file rather than have to manually set it yourself. We can automate this by following a few conventions:

| Convention          | Example                                                    | Result                                                       |
| ------------------- | ---------------------------------------------------------- | ------------------------------------------------------------ |
| Chapter index       | `00.md`                                                    | Represents the page for the parent folder                    |
| Sorted by filenames | `04_cli.md` or `06_docs/`                                  | Use leading numbers to sort pages (though not strictly enforced to have numbers in the filenames like this) |
| Partials            | `_shared.md`                                               | Partials start with an underscore and will be ignored in the summary |
| Page titles         | `# Page Title`                                             | Page names (rendered in the navigation) come from the first `H1` header of the page. An error is thrown if there is no title. |
| Prefix chapters     | `00_prologue.md`                                           | Prefix chapters start with `00` (excluding `00.md`)          |
| Suffix chapters     | `ZZ_final_words.md`                                        | Suffix chapters start with `ZZ`                              |
| Draft chapters      | `04_advanced_configuration().md` or `05_administration()/` | Draft pages and folders end with `()`                        |
| Separators          | `02__` or `02___________`                                  | Separators are files that end with two underscores `__`      |
| Part Titles         | `05_reference_#.md`                                        | Filenames ending in `#` indicate a part title and the title comes from the first `H1` header |

These conventions should create a filesystem structure that, when sorted alphanumerically, is the same in the final render.

```
00_prologue.md                    → prefix chapter
01_intro.md
02_install/
├─ 00.md                          → chapter index 
├─ 01_linux.md
├─ 02_mac.md                      ↓ files sorted naturally
├─ 03_windows.md
├─ 04_______                      → separator
├─ 05_post_install.md
├─ _common_install_tips.md        → ignored "partial"
03_caveat.md
04_##_guide_##.md                 → part title 
05_usage.md/
├─ 00.md
├─ 01_basics/
│  ├─ 00.md
│  ├─ 01_setup.md
│  ├─ 02_monitoring.md
06_administration()/              → draft chapter
├─ 00.md
├─ 01_install().md                → nested draft chapter
ZZ_final_words.md                 → suffix chapter
```

### Usage

You _must_ create a dummy `SUMMARY.md`, otherwise `mdbook` will error out before the preprocessors get called. (The contents aren't important. It can just have `# SUMMARY` as the first line.)

Install the project with cargo. The current version is <code>v<span id="version">0.2.0</span></code>.

```
cargo install mdbook-fs-summary
```

There are no configurable options right now.

```toml
# book.toml

[preprocessor.fs-summary]
```

### URL Handling and Number Prefixes

The number prefixes are mostly just a convention for keeping files and folders properly sorted. We don't rely on these numbers internally to generate a summary. You can opt out of having these numbers altogether; your files will just be sorted alphanumerically. 

Since these numbers are just our internal convention for sorting, we don't want to expose these numbers in the URLs produced by mdbook. So in order to strip them out, we have to specify what they are. 

Currently the convention is this: 

> If the filename starts with 2 or 3 numbers or upper case letters followed by an underscore, they'll get stripped in the resulting URLs. 

> ⚠️ This currently breaks support for the default links preprocessor provided by mdbook. [See this pull request.](https://github.com/rust-lang/mdBook/pull/1716)

### Alternatives

* [mdbook-auto-gen-summary](https://crates.io/crates/mdbook-auto-gen-summary) - Similar goals with different conventions and writes the resulting table of contents to `SUMMARY.md`. 
