import { createFileRoute } from '@tanstack/react-router';
import { AlertCircleIcon } from 'lucide-react';

import { commands } from '@/bindings';
import { PhotoThumbnail } from '@/components/app/PhotoThumbnail';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';

export const Route = createFileRoute('/people/$id')({
  component: RouteComponent,
  loader: async ({ params: { id } }) => {
    const person = await commands.getPerson(parseInt(id));
    const photos = await commands.searchPhotos({
      page: 1,
      per_page: 10,
      person_id: parseInt(id),
      text: null,
      threshold: null,
      country: null,
      country_id: null,
      city: null,
      city_id: null,
      date_from: null,
      date_to: null,
    });

    if (photos.status === 'error') throw new Error(photos.error);
    if (person.status === 'error') throw new Error(person.error);

    return {
      person,
      personPhotos: photos.data,
      breadcrumb: person.data.name,
    };
  },
  errorComponent: ({ error }: { error: { message: string } }) => {
    return (
      <Alert variant="destructive">
        <AlertCircleIcon />
        <AlertTitle>Error while loading person</AlertTitle>
        <AlertDescription>
          <p>{error.message}</p>
        </AlertDescription>
      </Alert>
    );
  },
});

function RouteComponent() {
  const { personPhotos } = Route.useLoaderData();
  return (
    <div>
      {personPhotos.items.map((p) => (
        <div key={p.path} className="w-2xs">
          <PhotoThumbnail photoPath={p.path} />
        </div>
      ))}
    </div>
  );
}
