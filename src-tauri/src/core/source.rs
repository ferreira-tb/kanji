use crate::database::model::source::Source;
use anyhow::Result;
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use itertools::Itertools;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::LazyLock;
use tap::Pipe;
use walkdir::{DirEntry, WalkDir};

static GLOBSET: LazyLock<GlobSet> = LazyLock::new(|| {
  GlobSetBuilder::new()
    .add(glob("*.md"))
    .add(glob("*.txt"))
    .build()
    .unwrap()
});

pub fn walk_source(source: &Source) -> Result<Vec<PathBuf>> {
  let mut files = WalkDir::new(&*source.path)
    .max_depth(1)
    .sort_by_file_name()
    .into_iter()
    .flatten()
    .map(DirEntry::into_path)
    .filter(|path| path.is_file() && GLOBSET.is_match(path))
    .collect_vec();

  let patterns = read_ignore_patterns(source)?;

  if !patterns.is_empty() {
    let mut globset = GlobSetBuilder::new();

    for pattern in patterns {
      if let Ok(glob) = Glob::new(pattern.as_str()) {
        globset.add(glob);
      }
    }

    let globset = globset.build()?;
    files.retain(|file| !globset.is_match(file));
  }

  Ok(files)
}

fn read_ignore_patterns(source: &Source) -> Result<Vec<String>> {
  let path = source.path.join(".kanjignore");
  let content = match fs::read_to_string(path) {
    Ok(it) => it,
    Err(err) => {
      if let ErrorKind::NotFound = err.kind() {
        return Ok(Vec::new());
      } else {
        return Err(err.into());
      }
    }
  };

  content
    .lines()
    .map(str::trim)
    .filter(|it| !it.is_empty())
    .map(ToOwned::to_owned)
    .collect_vec()
    .pipe(Ok)
}

fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}
