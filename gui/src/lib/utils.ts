import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { transliterate } from 'transliteration';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function createSmartFilter<T extends { id: number }>(items: T[], getValue: (item: T) => string) {
  return (value: string, search: string) => {
    const item = items.find((item) => String(item.id) === value);

    if (!item) return 0;

    const itemName = getValue(item);

    return transliterate(itemName).toLowerCase().includes(search.toLowerCase()) ? 1 : 0;
  };
}
