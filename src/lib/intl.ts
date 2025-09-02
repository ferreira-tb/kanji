export const percentIntl = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 0,
  minimumFractionDigits: 0,
  roundingMode: 'trunc',
  localeMatcher: 'best fit',
  style: 'percent',
});

export function formatPercent(num: number) {
  return percentIntl.format(num);
}
