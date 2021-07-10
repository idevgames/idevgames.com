export function toSimpleDate(date: Date): string {
  return `${date.getFullYear()}-${(date.getMonth() + 1).toLocaleString('en', { minimumIntegerDigits: 2 })}-${date.getDate().toLocaleString('en', { minimumIntegerDigits: 2 })}`;
}

export function fromSimpleDate(str: string): Date {
  return new Date(Date.parse(str));
}
