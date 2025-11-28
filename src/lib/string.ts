import { trimArray } from '@tb-dev/utils';

export function toChars(value: string) {
  const chars = Array.from(value.trim());
  return trimArray(chars, { allowEmpty: false });
}
