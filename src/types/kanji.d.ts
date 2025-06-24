interface Kanji {
  readonly character: KanjiChar;
  readonly seen: number;
  readonly ratio: number;
  readonly level: KanjiLevel;
  readonly sources: KanjiSource[];
}

type KanjiChar = string;

type KanjiLevel = 'common' | 'uncommon' | 'rare' | 'very-rare' | 'unknown';

interface KanjiSource {
  readonly name: string;
  readonly seen: number;
}
