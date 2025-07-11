import { createFileRoute } from '@tanstack/react-router';

import { commands } from '@/bindings';
import { ErrorMessage } from '@/components/app/ErrorMessage';
import { Loader } from '@/components/app/Loader';
import { PhotoGallery } from '@/components/app/PhotoGallery';
import { photoSearchSchema } from '@/photoSearch';

export const Route = createFileRoute('/search/gallery')({
  component: GalleryPage,
  validateSearch: photoSearchSchema,
  loaderDeps: ({ search }) => search,
  loader: async ({ deps }) => {
    const result = await commands.searchPhotos(deps);

    if (result.status === 'error') throw new Error(result.error);

    return {
      paginatedPhotos: result.data,
    };
  },
  pendingComponent: () => <Loader message={'Searching photos...'} />,
  errorComponent: ErrorMessage,
});

function GalleryPage() {
  const { paginatedPhotos } = Route.useLoaderData();

  return (
    <PhotoGallery
      photos={paginatedPhotos?.items || []}
      totalPages={paginatedPhotos?.total_pages || 0}
    />
  );
}
