import { createFileRoute, Outlet } from '@tanstack/react-router';
import { AlertCircleIcon } from 'lucide-react';

import { commands } from '@/bindings';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';

export const Route = createFileRoute('/people/$id')({
  component: RouteComponent,
  loader: async ({ params: { id } }) => {
    const person = await commands.getPerson(parseInt(id));

    if (person.status === 'error') throw new Error(person.error);

    return {
      person,
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
  const { person } = Route.useLoaderData();
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">{person.data.name}</h1>
      <Outlet />
    </div>
  );
}
