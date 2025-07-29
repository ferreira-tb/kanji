/* eslint-disable no-extend-native */
import { handleError } from '@tb-dev/vue';
import { PromiseSet } from '@tb-dev/utils';

Promise.prototype.chain = function(promise: Promise<unknown>) {
  return PromiseSet.from([this, promise]);
};

Promise.prototype.err = function() {
  this.catch((err: unknown) => handleError(err, /* rethrow */ false));
};
