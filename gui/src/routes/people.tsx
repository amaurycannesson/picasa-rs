import { createFileRoute, Outlet } from '@tanstack/react-router';

export const Route = createFileRoute('/people')({
  component: () => <Outlet />,
  loader: () => ({ breadcrumb: 'People' }),
});
