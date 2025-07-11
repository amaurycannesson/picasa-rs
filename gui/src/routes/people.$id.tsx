import { createFileRoute, Outlet } from '@tanstack/react-router';

import { commands } from '@/bindings';
import { ErrorMessage } from '@/components/app/ErrorMessage';

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
  errorComponent: ErrorMessage,
});

function RouteComponent() {
  const { person } = Route.useLoaderData();
  return (
    <div>
      <h1 className="mb-4 text-2xl font-bold">{person.data.name}</h1>
      <Outlet />
    </div>
  );
}
