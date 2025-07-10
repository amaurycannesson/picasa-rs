import { z } from 'zod';

import { PhotoSearchParams } from './bindings';

export const photoSearchSchema = z.object({
  text: z.string().min(3).nullable().default(null),
  threshold: z.number().min(0).max(1).nullable().default(null),
  country: z.string().min(3).nullable().default(null),
  country_id: z.number().min(0).nullable().default(null),
  city: z.string().min(3).nullable().default(null),
  city_id: z.number().min(0).nullable().default(null),
  date_from: z.string().min(3).nullable().default(null),
  date_to: z.string().min(3).nullable().default(null),
  person_id: z.number().min(0).nullable().default(null),
  page: z.number().min(1).default(1),
  per_page: z.number().min(1).default(15),
});

export const DEFAULT_PHOTO_SEARCH: PhotoSearchParams = photoSearchSchema.parse({});
