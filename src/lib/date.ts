import { ja } from 'date-fns/locale';
import { type DateArg, formatDate, formatDistanceToNow } from 'date-fns';

const ZONE_REGEX = /\[.+?\]/;

export function fromZoned(zoned: string) {
  return new Date(zoned.replace(ZONE_REGEX, ''));
}

export function formatZoned(zoned: string, format: string) {
  return formatDate(fromZoned(zoned), format);
}

export function since(timestamp: DateArg<Date>) {
  return formatDistanceToNow(timestamp, {
    locale: { formatDistance: ja.formatDistance },
  });
}

export function sinceZoned(zoned: string) {
  return since(fromZoned(zoned));
}
