import { createFileRoute } from '@tanstack/react-router';

import { commands, PhotoSearchParams } from '@/bindings';
import { PhotoGallery } from '@/components/app/PhotoGallery';

export const Route = createFileRoute('/people/$id/gallery')({
  component: GalleryPage,
  validateSearch: (search: Record<string, unknown>): PhotoSearchParams => ({
    page: Number(search?.page ?? 1),
    per_page: Number(search?.per_page ?? 20),
    text: null,
    threshold: null,
    country: null,
    city: null,
    date_from: null,
    date_to: null,
    country_id: null,
    city_id: null,
    person_id: null,
  }),
  loaderDeps: ({ search }) => search,
  loader: async ({ params: { id }, deps }) => {
    const params = {
      ...deps,
      person_id: parseInt(id),
      page: deps.page,
      per_page: deps.per_page,
    };

    const result = await commands.searchPhotos(params);

    if (result.status === 'error') throw new Error(result.error);

    return {
      paginatedPhotos: result.data,
    };
  },
  pendingComponent: () => <>Loading...</>,
});

function GalleryPage() {
  const { id } = Route.useParams();
  const search = Route.useSearch();
  const { paginatedPhotos } = Route.useLoaderData();

  const getNavigationConfig = (page: number) => ({
    to: '/people/$id/gallery' as const,
    params: { id },
    search: { ...search, page },
  });

  return (
    <PhotoGallery
      photos={paginatedPhotos?.items || []}
      currentPage={search.page}
      totalPages={paginatedPhotos?.total_pages || 0}
      getNavigationConfig={getNavigationConfig}
    />
  );
}
