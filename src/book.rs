use anyhow::{Context, Result};
use mdbook::book::{Link, SectionNumber, Summary, SummaryItem};
use mdbook::preprocess::PreprocessorContext;
use mdbook::MDBook;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_book(ctx: &PreprocessorContext) -> Result<MDBook> {
    let summary = load_summary(ctx.config.book.src.as_path())?;
    let book =
        MDBook::load_with_config_and_summary(ctx.root.as_path(), ctx.config.clone(), summary)?;
    Ok(book)
}

fn load_summary(book_src: &Path) -> Result<Summary> {
    let mut numbered_chapters = load_summary_items(book_src, book_src)?;
    apply_section_numbers(&mut numbered_chapters, &Vec::default());
    Ok(Summary {
        title: None,
        prefix_chapters: Default::default(),
        numbered_chapters,
        suffix_chapters: Default::default(),
    })
}

fn apply_section_numbers(chapters: &mut [SummaryItem], parent_num: &Vec<u32>) {
    let mut i = 0_u32;
    for chapter in chapters {
        i += 1;
        if let SummaryItem::Link(ref mut link) = *chapter {
            let mut num = parent_num.clone();
            num.push(i);
            apply_section_numbers(&mut link.nested_items, &num);
            link.number = Some(SectionNumber(num));
        }
    }
}

fn load_summary_items<P: AsRef<Path>>(path: P, book_src: &Path) -> Result<Vec<SummaryItem>> {
    // We can't say we're getting the directory contents in order. That means we have to sort them
    // ourselves. Using a BTreeMap gives us that, but also it means the whole tree won't be in
    // order until _after_ it's all built. That means we can't apply section numbers at this point.
    let mut map = BTreeMap::default();
    let summary_path: PathBuf = [PathBuf::from(book_src), PathBuf::from("SUMMARY.md")]
        .iter()
        .collect();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path().to_path_buf();
        if path == summary_path {
            continue;
        }
        if let Some(item) = load_summary_item(entry, book_src)? {
            map.insert(path, item);
        }
    }
    Ok(map.values().cloned().collect())
}

fn load_summary_item(entry: fs::DirEntry, book_src: &Path) -> Result<Option<SummaryItem>> {
    let ft = entry.file_type()?;
    if ft.is_dir() {
        let nested_items = load_summary_items(entry.path(), book_src)?;

        let index_file: PathBuf = [entry.path(), PathBuf::from("00.md")].iter().collect();

        if !index_file.exists() {

            // directories with no markdown files are skipped (might contain other assets)
            if nested_items.is_empty() {
                return Ok(None);
            }

            return Err(anyhow::Error::msg(format!(
                "missing folder index file: {}",
                index_file.display()
            )));
        }

        let location = index_file.strip_prefix(book_src)?;

        return Ok(Some(SummaryItem::Link(Link {
            name: load_summary_title(index_file.as_path())?,
            location: Some(location.to_path_buf()),
            number: None, // updated later after tree is sorted properly
            nested_items,
        })));
    }
    if ft.is_file() {
        let os_filename = entry.file_name();
        let filename = os_filename.to_string_lossy();

        // skip folder index files (already added when we added the directory)
        if filename == "00.md" {
            return Ok(None);
        }
        // skip partials
        if filename.starts_with("_") {
            return Ok(None);
        }
        // skip non-markdown files
        if !filename.ends_with(".md") {
            return Ok(None);
        }

        let path = entry.path().to_path_buf();
        let location = path.strip_prefix(book_src)?;

        return Ok(Some(SummaryItem::Link(Link {
            name: load_summary_title(entry.path())?,
            location: Some(location.to_path_buf()),
            number: None, // we're ignoring numbers anyway
            nested_items: Default::default(),
        })));
    }
    Ok(None)
}

fn load_summary_title<P: AsRef<Path>>(path: P) -> Result<String> {
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
