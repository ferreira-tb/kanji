use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use walkdir::WalkDir;

static GLOBSET: LazyLock<GlobSet> = LazyLock::new(|| {
  GlobSetBuilder::new()
    .add(glob("*.md"))
    .add(glob("*.txt"))
    .build()
    .unwrap()
});

fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}

pub fn walk_dir(dir: &Path) -> Vec<PathBuf> {
  WalkDir::new(dir)
    .into_iter()
    .flatten()
    .map(|entry| entry.into_path())
    .filter(|path| path.is_file() && GLOBSET.is_match(&path))
    .collect()
}
