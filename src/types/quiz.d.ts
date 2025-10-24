type Quiz = readonly QuizQuestion[];

type QuizKind = QuizKindChunk | QuizKindRandomChunk | QuizKindSource | QuizKindRandomSource;

interface QuizKindChunk {
  readonly kind: 'chunk';
  readonly chunk: readonly KanjiChar[];
}

interface QuizKindRandomChunk {
  readonly kind: 'random-chunk';
}

interface QuizKindSource {
  readonly kind: 'source';
  readonly id: SourceId;
}

interface QuizKindRandomSource {
  readonly kind: 'random-source';
}

interface QuizQuestion {
  readonly snippet: Snippet;
  readonly censored: string;
  readonly answer: KanjiChar;
  readonly options: readonly KanjiChar[];
}

interface QuizAnswer {
  readonly id: QuizAnswerId;
  readonly question: KanjiChar;
  readonly answer: KanjiChar;
  readonly createdAt: string;
  readonly sourceId: Option<SourceId>;
}

type QuizAnswerId = string;
