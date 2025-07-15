import { z } from 'zod';

import { PhotoSearchParams } from './bindings';

export const photoSearchSchema = z.object({
  text: z
    .string()
    .nullable()
    .default(null)
    .transform((val) => (val === '' ? null : val)),
  threshold: z.number().min(0).max(1).nullable().default(null),
  country: z.string().nullable().default(null),
  country_id: z.coerce
    .number()
    .nullable()
    .default(null)
    .transform((val) => (val === 0 ? null : val)),
  city: z.string().nullable().default(null),
  city_id: z.coerce
    .number()
    .nullable()
    .default(null)
    .transform((val) => (val === 0 ? null : val)),
  date_from: z
    .string()
    .nullable()
    .default(null)
    .transform((val) => (val === '' ? null : val)),
  date_to: z
    .string()
    .nullable()
    .default(null)
    .transform((val) => (val === '' ? null : val)),
  person_ids: z
    .array(z.coerce.number())
    .nullable()
    .default(null)
    .transform((val) => (val && val.length === 0 ? null : val)),
  person_match_mode: z.enum(['All', 'Any']).nullable().default(null),
  page: z.number().min(1).default(1),
  per_page: z.number().min(1).default(15),
});

export const DEFAULT_PHOTO_SEARCH: PhotoSearchParams = photoSearchSchema.parse({});
