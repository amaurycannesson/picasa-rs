import { isMatch, Link, useMatches } from '@tanstack/react-router';

import {
  Breadcrumb as UiBreadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
} from '../ui/breadcrumb';

const Breadcrumb = () => {
  const matches = useMatches();

  const items = matches
    .filter((match) => isMatch(match, 'loaderData.breadcrumb'))
    .map(({ pathname, loaderData }) => {
      return {
        href: pathname,
        label: loaderData?.breadcrumb,
      };
    });

  return (
    <UiBreadcrumb>
      <BreadcrumbList>
        {items.map(({ label, href }) => (
          <BreadcrumbItem key={href} className="hidden md:block">
            <BreadcrumbLink asChild={true}>
              <Link to={href}>{label}</Link>
            </BreadcrumbLink>
          </BreadcrumbItem>
        ))}
      </BreadcrumbList>
    </UiBreadcrumb>
  );
};

export { Breadcrumb };
