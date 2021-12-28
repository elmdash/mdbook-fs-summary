# mdbook-fs-summary

A preprocessor that generates a summary from the file structure of your book rather than using an explicit `SUMMARY.md` file.

### Benefits

Frequently, you want the organization of your files to simply be mirrored in the `SUMMARY.md` file rather than have to manually set it yourself. We can automate this by following a few conventions:

- A page that represents the parent folder is called `00.md`
- Pages and folders are ordered by the numbers in the filename, i.e. `04_cli.md` comes before  `05_agile.md` and the `05_team` folder comes before the `06_docs` folder.
- Partials start with an underscore (i.e. `_shared.md`) and should not be included in the table of contents. You can also use an underscore to create a "draft".
- Page names (rendered in the navigation) come from the first `H1` header of the page. An error is thrown if there is no title.
- There are no prefix or suffix chapters right now.
- There are no separators either.

These conventions should create a filesystem structure that, when sorted alphanumerically, is the same in the final render.

```
01_intro.md
02_install.md/
├─ 00.md
├─ 01_linux.md
├─ 02_mac.md
├─ 03_windows.md
├─ _common_install_tips.md
03_caveat.md
04_usage.md/
├─ 00.md
├─ 01_basics.md/
│  ├─ 00.md
│  ├─ 01_setup.md
│  ├─ 02_monitoring.md
```

### Usage

You _must_ create a dummy `SUMMARY.md`, otherwise `mdbook` will error out before the preprocessors get called. (The contents aren't important. It can just have `# SUMMARY` as the first line.)

```
cargo install mdbook-fs-summary
```

There are no configurable options right now.

```toml
# book.toml

[preprocessor.fs-summary]
```

### Alternatives

* [mdbook-auto-gen-summary](https://crates.io/crates/mdbook-auto-gen-summary) - Similar goals with different conventions and writes the resulting table of contents to`SUMMARY.md`
