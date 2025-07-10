import { createFileRoute, redirect } from '@tanstack/react-router';

import { DEFAULT_PHOTO_SEARCH } from '@/photoSearch';

export const Route = createFileRoute('/')({
  component: () => <></>,
  beforeLoad: () => {
    throw redirect({
      to: '/search',
      search: DEFAULT_PHOTO_SEARCH,
    });
  },
});
