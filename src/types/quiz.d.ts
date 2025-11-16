type Quiz = readonly QuizQuestion[];

type QuizKind =
  | QuizKindChunk
  | QuizKindRandomChunk
  | QuizKindSource
  | QuizKindRandomSource
  | QuizKindSourceGroup
  | QuizKindRandomSourceGroup;

interface QuizKindChunk {
  readonly kind: 'chunk';
  readonly chunk: readonly KanjiChar[];
}

interface QuizKindRandomChunk {
  readonly kind: 'random-chunk';
}

interface QuizKindSource {
  readonly kind: 'source';
  readonly ids: readonly SourceId[];
}

interface QuizKindRandomSource {
  readonly kind: 'random-source';
}

interface QuizKindSourceGroup {
  readonly kind: 'source-group';
  readonly ids: readonly SourceGroupId[];
}

interface QuizKindRandomSourceGroup {
  readonly kind: 'random-source-group';
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

interface QuizSourceStats {
  readonly source: SourceId;
  readonly quizzes: number;
  readonly correctQuizAnswers: number;
  readonly quizAccuracy: number;
}
