type Route = 'home' | 'settings';

interface Kanji {
  character: KanjiChar;
  seen: number;
  ratio: number;
  level: Level;
  sources: Source[];
}

type KanjiChar = string;

type Level = 'common' | 'uncommon' | 'rare' | 'very-rare' | 'unknown';

interface Source {
  name: string;
  seen: number;
}

interface Sorting {
  ascending: boolean;
}
