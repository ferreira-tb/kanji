interface Settings {
  readonly clipboard: boolean;
  readonly editor: Editor;
  readonly hideOnClose: boolean;

  readonly snippetLimit: number;
  readonly snippetMinLen: number;
  readonly shuffleSnippets: boolean;
  readonly ignoreSourceWeight: boolean;

  readonly setFileName: string;
  readonly setChunkSize: number;
}

type Editor = 'code' | 'code-insiders' | 'zed';
