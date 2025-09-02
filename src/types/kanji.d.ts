type KanjiChar = string;

interface KanjiStats {
  readonly character: KanjiChar;
  readonly seen: number;
  readonly ratio: number;
  readonly level: KanjiLevel;
  readonly sources: KanjiStatsSource[];
  readonly quizzes: number;
  readonly correctQuizAnswers: number;
  readonly quizAccuracy: number;
}

interface KanjiStatsSource {
  readonly id: SourceId;
  readonly name: string;
  readonly seen: number;
}

type KanjiLevel = 'common' | 'uncommon' | 'rare' | 'very-rare' | 'unknown';
