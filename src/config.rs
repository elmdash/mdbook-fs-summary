use lazy_static::lazy_static;
use mdbook::preprocess::PreprocessorContext;
use regex::Regex;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};

pub const PREPROCESSOR_NAME: &str = "fs-summary";

pub struct Config {
    clean_paths: bool,
}

impl Config {
    pub fn new(ctx: &PreprocessorContext) -> Self {
        let mut clean_paths = true;
        if let Some(cfg) = ctx.config.get_preprocessor(PREPROCESSOR_NAME) {
            if let Some(val) = cfg.get("clean-paths") {
                clean_paths = val.as_bool().unwrap_or(true);
            }
        }

        Self { clean_paths }
    }

    /// Removes indicators from the path (like question marks).
    /// This allows for showing a different path than the actual source path.
    pub fn clean_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        if !self.clean_paths {
            return path.as_ref().to_path_buf();
        }
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
        let conf = Config { clean_paths: true };
        let p = PathBuf::from("02_hmm/05_here()/02_sure()/06_normal.md");
        assert_eq!(conf.clean_path(p), PathBuf::from("hmm/here/sure/normal.md"))
    }
}
