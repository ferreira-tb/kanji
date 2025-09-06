interface Source {
  readonly id: SourceId;
  readonly path: string;
  name: string;
  enabled: boolean;
  weight: SourceWeight;
}

type SourceId = number;
type SourceWeight = number;
