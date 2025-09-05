type SourceId = number;

interface Source {
  readonly id: SourceId;
  readonly path: string;
  name: string;
  enabled: boolean;
}
