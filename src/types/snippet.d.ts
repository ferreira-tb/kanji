interface Snippet {
  readonly id: string;
  readonly content: string;
  readonly source: SnippetSource;
}

interface SnippetSource {
  readonly name: string;
  readonly path: string;
  readonly line: number;
}
