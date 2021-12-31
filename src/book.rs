use anyhow::{Context, Result};
use lazy_static::lazy_static;
use mdbook::book::{Book, BookItem, Chapter, SectionNumber};
use mdbook::preprocess::PreprocessorContext;
use regex::Regex;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub fn load_book(ctx: &PreprocessorContext) -> Result<Book> {
    let root = ctx.config.book.src.as_path();

    let mut sections = load_book_items(root, &Vec::default(), root)?;
    apply_section_numbers(&mut sections, &Vec::default());
    let mut book = Book::default();
    for section in sections {
        book.push_item(section);
    }

    Ok(book)
}

fn apply_section_numbers(chapters: &mut [BookItem], parent_num: &Vec<u32>) {
    let mut i = 0_u32;
    for chapter in chapters {
        match chapter {
            BookItem::PartTitle(title) => {
                if !parent_num.is_empty() {
                    eprintln!("Warning: nested part titles may not be supported {}", title);
                }
            }
            BookItem::Chapter(ref mut chap) => {
                if chap.number.is_some() {
                    if !parent_num.is_empty() {
                        eprintln!(
                            "Warning: prefix and suffix chapters nested under numbered chapters may \
                        not be supported: {}",
                            chap.source_path
                                .as_ref()
                                .expect(
                                    "there should always be a source file for prefix/suffix chapters"
                                )
                                .display()
                        );
                    }
                    chap.number = None;
                    continue;
                }
                i += 1;
                let mut num = parent_num.clone();
                num.push(i);
                apply_section_numbers(&mut chap.sub_items, &num);
                chap.number = Some(SectionNumber(num));
            }
            _ => {
                continue;
            }
        }
    }
}

fn load_book_items<P: AsRef<Path>>(
    path: P,
    crumbs: &Vec<String>,
    book_src: &Path,
) -> Result<Vec<BookItem>> {
    // We can't say we're getting the directory contents in order. That means we have to sort them
    // ourselves. Using a BTreeMap gives us that, but also it means the whole tree won't be in
    // order until _after_ it's all built. That means we can't apply section numbers at this point.
    let mut map = BTreeMap::default();
    let summary_path = book_src.join(PathBuf::from("SUMMARY.md"));
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path().to_path_buf();
        if path == summary_path {
            continue;
        }
        if let Some(item) = load_book_item(entry, crumbs, book_src)? {
            map.insert(path, item);
        }
    }
    Ok(map.values().cloned().collect())
}

fn load_book_item(
    entry: fs::DirEntry,
    crumbs: &Vec<String>,
    book_src: &Path,
) -> Result<Option<BookItem>> {
    let ft = entry.file_type()?;
    if ft.is_dir() {
        let path = entry.path();

        let index_file = path.join(PathBuf::from("00.md"));
        if !index_file.exists() {
            // directories with no markdown files are skipped (might contain other assets)
            // so it may not be an error not to have an index file anyway
            let found_items = load_book_items(entry.path(), &Vec::default(), book_src)?;
            if found_items.is_empty() {
                return Ok(None);
            }

            return Err(anyhow::Error::msg(format!(
                "missing folder index file: {}",
                index_file.display()
            )));
        }

        let dir_name = if let Some(f) = path.file_name() {
            f.to_string_lossy()
        } else {
            // skip directories with invalid names
            return Ok(None);
        };

        let name = load_chapter_title(index_file.as_path())?;
        let mut parent_names = crumbs.clone();
        parent_names.push(name.clone());
        let sub_items = load_book_items(entry.path(), &parent_names, book_src)?;

        if dir_name.ends_with("()") {
            return Ok(Some(BookItem::Chapter({
                let mut c = Chapter::new_draft(&name, parent_names);
                c.sub_items = sub_items;
                c
            })));
        }

        let content = fs::read_to_string(index_file.as_path())
            .with_context(|| format!("could not read file: {}", index_file.as_path().display()))?;

        let source_path = index_file.strip_prefix(book_src)?;
        let mut cleaned_path = clean_path(source_path);
        cleaned_path.set_file_name("index.md");

        return Ok(Some(BookItem::Chapter({
            let mut c = Chapter::new(&name, content, source_path, parent_names);
            c.sub_items = sub_items;
            c.path = Some(cleaned_path);
            c
        })));
    }
    if ft.is_file() {
        // for numbered chapters, we initially define it as `None` as they are calculated later
        let mut number = None;
        let path = entry.path().to_path_buf();

        let filename = if let Some(f) = path.file_name() {
            f.to_string_lossy()
        } else {
            // skip files that don't have valid filenames
            return Ok(None);
        };
        let base_filename = if let Some(f) = path.file_stem() {
            f.to_string_lossy()
        } else {
            // skip files that don't have valid filenames
            return Ok(None);
        };

        if base_filename.ends_with("__") {
            return Ok(Some(BookItem::Separator));
        }

        if base_filename.ends_with("#") {
            return Ok(Some(BookItem::PartTitle(load_chapter_title(
                path.as_path(),
            )?)));
        }

        // skip partials
        if filename.starts_with("_") {
            return Ok(None);
        }
        // skip non-markdown files
        if let Some(ext) = entry.path().extension() {
            if ext != "md" {
                return Ok(None);
            }
        }
        // skip folder index files (already added when we added the directory)
        if filename == "00.md" {
            return Ok(None);
        }

        if filename.starts_with("00") || filename.starts_with("ZZ") {
            // little hacky, but we're indicating this has no section number by setting it now
            // and then unsetting it later. otherwise we need a custom newtype of `BookItem`
            number = Some(SectionNumber::default());
        }

        let name = load_chapter_title(path.as_path())?;

        if base_filename.ends_with("()") {
            return Ok(Some(BookItem::Chapter({
                let mut c = Chapter::new_draft(&name, crumbs.clone());
                c.number = number;
                c
            })));
        }

        let source_path = path.strip_prefix(book_src)?;
        let content = fs::read_to_string(path.as_path())
            .with_context(|| format!("could not read file: {}", path.as_path().display()))?;

        return Ok(Some(BookItem::Chapter({
            let mut c = Chapter::new(&name, content, source_path, crumbs.clone());
            c.path = Some(clean_path(source_path));
            c.number = number;
            c
        })));
    }
    Ok(None)
}

/// Reads the first H1 header in a markdown file to get the title of the chapter
fn load_chapter_title<P: AsRef<Path>>(path: P) -> Result<String> {
    let contents = fs::read_to_string(path.as_ref())
        .with_context(|| format!("could not read file: {}", path.as_ref().display()))?;
    let mut title = None;
    for l in contents.lines() {
        if l.starts_with("# ") {
            if let Some(t) = l.get(2..) {
                title = Some(String::from(t))
            }
            break;
        }
    }
    title.ok_or_else(|| {
        anyhow::Error::msg(format!(
            "could not find H1 title from: {}",
            path.as_ref().display()
        ))
    })
}

/// Removes indicators from the path (like question marks).
/// This allows for showing a different path than the actual source path.
fn clean_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut out = PathBuf::new();
    for part in path.as_ref().components() {
        if let Component::Normal(val) = part {
            if let Some(val_str) = val.to_str() {
                let mut cleaned = String::from(val_str);
                strip_draft_indicator(&mut cleaned);
                strip_numbering_prefix(&mut cleaned);
                out.push(OsString::from(cleaned));
                continue;
            }
        }

        out.push(OsString::from(part.as_os_str()));
    }
    out
}

fn strip_draft_indicator(s: &mut String) {
    if let Some(stripped) = s.strip_suffix("()") {
        *s = stripped.to_string();
    }
}

fn strip_numbering_prefix(s: &mut String) {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9A-Z]{2,3}_").unwrap();
    }

    let stripped = RE.replace(s, "");
    *s = String::from(stripped);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_drafts() {
        let p = PathBuf::from("02_hmm/05_here()/02_sure()/06_normal.md");
        assert_eq!(clean_path(p), PathBuf::from("hmm/here/sure/normal.md"))
    }
}
