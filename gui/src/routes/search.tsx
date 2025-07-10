import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';

import { commands, PhotoSearchParams } from '@/bindings';
import { PhotoGallery } from '@/components/app/PhotoGallery';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

export const Route = createFileRoute('/search')({
  component: SearchPage,
  validateSearch: (search: Record<string, unknown>): PhotoSearchParams => ({
    page: Number(search?.page ?? 1),
    per_page: Number(search?.per_page ?? 20),
    text: (search?.text as string) || null,
    threshold: null,
    country: (search?.country as string) || null,
    city: (search?.city as string) || null,
    date_from: null,
    date_to: null,
    country_id: null,
    city_id: null,
    person_id: null,
  }),
  loaderDeps: ({ search }) => search,
  loader: async ({ deps }) => {
    const result = await commands.searchPhotos(deps);

    if (result.status === 'error') throw new Error(result.error);

    return {
      breadcrumb: 'Photo search',
      paginatedPhotos: result.data,
    };
  },
  pendingComponent: () => <>Loading...</>,
});

function SearchPage() {
  const search = Route.useSearch();
  const { paginatedPhotos } = Route.useLoaderData();
  const navigate = Route.useNavigate();
  const [searchText, setSearchText] = useState(search.text || '');
  const [searchCountry, setSearchCountry] = useState(search.country || '');
  const [searchCity, setSearchCity] = useState(search.city || '');

  function handleSearch() {
    navigate({
      to: '/search',
      search: {
        ...search,
        text: searchText || null,
        country: searchCountry || null,
        city: searchCity || null,
        page: 1,
      },
    });
  }

  const getNavigationConfig = (page: number) => ({
    to: '/search' as const,
    search: { ...search, page },
  });

  return (
    <div>
      <div className="mb-4 flex gap-2">
        <Input
          placeholder="Search..."
          value={searchText}
          onChange={(e) => setSearchText(e.target.value)}
          className="flex-1"
        />
        <Input
          placeholder="Country..."
          value={searchCountry}
          onChange={(e) => setSearchCountry(e.target.value)}
          className="flex-1"
        />
        <Input
          placeholder="City..."
          value={searchCity}
          onChange={(e) => setSearchCity(e.target.value)}
          className="flex-1"
        />
        <Button onClick={handleSearch}>Search</Button>
      </div>

      <PhotoGallery
        photos={paginatedPhotos?.items || []}
        currentPage={search.page}
        totalPages={paginatedPhotos?.total_pages || 0}
        getNavigationConfig={getNavigationConfig}
      />
    </div>
  );
}
