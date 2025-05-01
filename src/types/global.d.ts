import type { PromiseSet } from '@tb-dev/utils';

declare global {
  interface Promise<T> {
    chain: (promise: Promise<unknown>) => PromiseSet;
    err: () => void;
  }
}
