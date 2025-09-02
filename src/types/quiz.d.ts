type Quiz = readonly QuizQuestion[];

interface QuizQuestion {
  readonly snippet: Snippet;
  readonly censored: string;
  readonly answer: KanjiChar;
  readonly options: readonly KanjiChar[];
}
