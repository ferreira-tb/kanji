/* eslint-disable no-inner-declarations */
/* eslint-disable @typescript-eslint/naming-convention */
import type { PromiseSet } from '@tb-dev/utils';

declare global {
  var __DEBUG_ASSERTIONS__: boolean;

  interface Promise<T> {
    chain: (promise: Promise<unknown>) => PromiseSet;
    err: () => void;
  }
}
