type KanjiSet = readonly KanjiSetChunk[];

interface KanjiSetChunk {
  readonly id: KanjiSetChunkId;
  readonly kanjis: readonly KanjiChar[];
}

type KanjiSetChunkId = number;
