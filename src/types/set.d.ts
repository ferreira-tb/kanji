interface KanjiSet {
  readonly chunks: readonly KanjiSetChunk[];
  readonly quizzes: number;
  readonly correctQuizAnswers: number;
  readonly quizAccuracy: number;
}

interface KanjiSetChunk {
  readonly id: KanjiSetChunkId;
  readonly kanjis: readonly KanjiChar[];
  readonly quizzes: number;
  readonly correctQuizAnswers: number;
  readonly quizAccuracy: number;
}

type KanjiSetChunkId = number;
