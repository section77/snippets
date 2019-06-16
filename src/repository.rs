use crate::{Snippet, SnippetId, Tags};
use std::convert::AsRef;
use std::fs;
use std::io;
use std::path;

pub trait Repository {
    fn next_id(&self) -> io::Result<SnippetId>;
    fn create(&self, tags: Tags, content: &str) -> io::Result<Snippet>;
    fn list(&self) -> io::Result<Vec<Snippet>>;
}

#[derive(Debug)]
pub struct FileRepository {
    path: path::PathBuf,
}

impl FileRepository {
    pub fn new(path: &path::Path) -> Self {
        FileRepository {
            path: path.to_path_buf(),
        }
    }
}

impl Repository for FileRepository {
    fn next_id(&self) -> io::Result<SnippetId> {
        let entries = fs::read_dir(&self.path)?;
        let last_id = entries
            .filter_map(|e| {
                let path = e.ok()?.path();
                path.file_stem().and_then(|basename_without_ext| {
                    basename_without_ext.to_str()?.parse::<u64>().ok()
                })
            })
            .max()
            .unwrap_or(0);
        Ok(last_id + 1)
    }

    fn create(&self, tags: Tags, content: &str) -> io::Result<Snippet> {
        let id = self.next_id()?;

        let snippet = Snippet::new(id, tags, content.as_ref());

        let file_path = self.path.join(format!("{}.json", id));
        let json = serde_json::to_string(&snippet)?;
        fs::write(file_path, json)?;
        Ok(snippet)
    }

    fn list(&self) -> io::Result<Vec<Snippet>> {
        let entries = fs::read_dir(&self.path)?;

        // FIXME: invalid values are currently discarded
        let mut snippets: Vec<Snippet> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let raw: String = fs::read_to_string(&path).ok()?;
                match serde_json::from_str(&raw) {
                    Ok(snippet) => Some(snippet),
                    Err(err) => {
                        println!(
                            "unable to deserialize 'Snippet' from file: {:?}, raw: {}, err: {}",
                            path, raw, err
                        );
                        None
                    }
                }
            })
            .collect();

        snippets.sort_unstable_by(|a, b| a.ts.partial_cmp(&b.ts).unwrap().reverse());
        Ok(snippets)
    }
}
