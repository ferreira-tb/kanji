use crate::database::model::source::Source;
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use itertools::Itertools;
use std::path::PathBuf;
use std::sync::LazyLock;

static GLOBSET: LazyLock<GlobSet> = LazyLock::new(|| {
  GlobSetBuilder::new()
    .add(glob("*.md"))
    .add(glob("*.txt"))
    .build()
    .unwrap()
});

pub fn walk_source(source: &Source) -> Vec<PathBuf> {
  WalkBuilder::new(&*source.path)
    .add_custom_ignore_filename(".kanjignore")
    .current_dir(&*source.path)
    .follow_links(false)
    .standard_filters(false)
    .max_depth(Some(1))
    .sort_by_file_name(Ord::cmp)
    .build()
    .flatten()
    .map(DirEntry::into_path)
    .filter(|path| path.is_file() && GLOBSET.is_match(path))
    .collect_vec()
}

fn glob(glob: &str) -> Glob {
  GlobBuilder::new(glob)
    .case_insensitive(true)
    .build()
    .unwrap()
}
