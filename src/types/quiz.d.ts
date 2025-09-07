type Quiz = readonly QuizQuestion[];

interface QuizQuestion {
  readonly snippet: Snippet;
  readonly censored: string;
  readonly answer: KanjiChar;
  readonly options: readonly KanjiChar[];
}

interface QuizAnswer {
  readonly id: string;
  readonly question: KanjiChar;
  readonly answer: KanjiChar;
  readonly createdAt: string;
}
