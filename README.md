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

Install the project with cargo. The current version is <code>v<span id="version">0.2.1</span></code>.

```
cargo install mdbook-fs-summary
```

There aren't a lot of configurations right now.

```toml
# book.toml

[preprocessor.fs-summary]
# (default: true)
clean-paths = true

# other preprocessors will naturally need to 
# run after the summary has been generated 
[preprocessor.links]
after = ["fs-summary"] 
```

### Clean Paths

We are using filename indicators to apply configurations to the generated summary. That's not ideal, so they are stripped to their natural paths.

Instead of: 

```
http://localhost:3000/02_team/05_onboarding.html
```

We strip the numeric prefixes used for sorting and any other artificial indicators.

```
http://localhost:3000/team/onboarding.html
```

> ⚠️ Cleaning paths currently breaks support for the default links preprocessor provided by mdbook. [See this pull request.](https://github.com/rust-lang/mdBook/pull/1716) It is recommended to disable clean paths until this is resolved if you intend to use partials.

##### Disable Clean Paths

You can turn off this behavior. 

```toml
[preprocessor.fs-summary]
clean-paths = false

[preprocessor.links]
after = ["fs-summary"]
```

##### Numeric Prefix Format

This processor doesn't dictate any special format for numeric prefixes used for sorting except when cleaning paths. Currently the convention is this: 

> If the filename starts with 2 or 3 numbers or upper case letters followed by an underscore, they'll get stripped in the resulting URLs. 

This is just something to note if you plan on some other sorting strategy for your filenames.

### Alternatives

* [mdbook-auto-gen-summary](https://crates.io/crates/mdbook-auto-gen-summary) - Similar goals with different conventions and writes the resulting table of contents to `SUMMARY.md`. 
