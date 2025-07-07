import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/people')({
  component: RouteComponent,
  staticData: { breadcrumb: 'People' },
});

function RouteComponent() {
  return <div>Hello /people!</div>;
}
