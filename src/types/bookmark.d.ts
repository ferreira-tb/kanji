interface Bookmark {
  readonly id: BookmarkId;
  readonly snippet: string;
  readonly sourceId: SourceId;
  readonly createdAt: string;
}

type BookmarkId = number;
