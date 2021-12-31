use anyhow::{Context, Result};
use mdbook::book::{Book, BookItem, Chapter, SectionNumber};
use mdbook::preprocess::PreprocessorContext;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

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
        i += 1;
        if let BookItem::Chapter(ref mut chap) = *chapter {
            let mut num = parent_num.clone();
            num.push(i);
            apply_section_numbers(&mut chap.sub_items, &num);
            chap.number = Some(SectionNumber(num));
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
        let index_file = entry.path().join(PathBuf::from("00.md"));
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

        let name = load_chapter_title(index_file.as_path())?;
        let mut parent_names = crumbs.clone();
        parent_names.push(name.clone());
        let sub_items = load_book_items(entry.path(), &parent_names, book_src)?;

        let content = fs::read_to_string(index_file.as_path())
            .with_context(|| format!("could not read file: {}", index_file.as_path().display()))?;

        let source_path = index_file.strip_prefix(book_src)?;

        return Ok(Some(BookItem::Chapter(Chapter {
            name,
            content,
            number: None, // updated later after tree is sorted properly
            sub_items,
            path: Some(source_path.clone().to_path_buf()),
            source_path: Some(source_path.to_path_buf()),
            parent_names,
        })));
    }
    if ft.is_file() {
        let os_filename = entry.file_name();
        let filename = os_filename.to_string_lossy();

        // skip folder index files (already added when we added the directory)
        if filename == "00.md" {
            return Ok(None);
        }
        // separators
        if filename.ends_with("__") {
            return Ok(Some(BookItem::Separator));
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
        let source_path = path.strip_prefix(book_src)?;
        let content = fs::read_to_string(path.as_path())
            .with_context(|| format!("could not read file: {}", path.as_path().display()))?;

        return Ok(Some(BookItem::Chapter(Chapter {
            name: load_chapter_title(entry.path())?,
            content,
            number: None, // we're ignoring numbers anyway
            sub_items: Default::default(),
            path: Some(source_path.clone().to_path_buf()),
            source_path: Some(source_path.to_path_buf()),
            parent_names: crumbs.clone(),
        })));
    }
    Ok(None)
}

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
