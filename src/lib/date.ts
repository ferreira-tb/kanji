import { formatDate } from 'date-fns';

const ZONE_REGEX = /\[.+?\]/;

export function fromZoned(zoned: string) {
  return new Date(zoned.replace(ZONE_REGEX, ''));
}

export function formatZoned(zoned: string, format: string) {
  return formatDate(fromZoned(zoned), format);
}
