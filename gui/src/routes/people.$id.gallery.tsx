import { createFileRoute } from '@tanstack/react-router';

import { commands } from '@/bindings';
import { PhotoGallery } from '@/components/app/PhotoGallery';
import { photoSearchSchema } from '@/photoSearch';

export const Route = createFileRoute('/people/$id/gallery')({
  component: GalleryPage,
  validateSearch: photoSearchSchema,
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
