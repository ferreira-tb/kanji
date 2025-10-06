interface Snippet {
  readonly id: SnippetId;
  readonly content: string;
  readonly source: SnippetSource;
  bookmark: Option<BookmarkId>;
}

type SnippetId = string;

interface SnippetSource {
  readonly id: SourceId;
  readonly name: string;
  readonly path: string;
  readonly line: number;
}
