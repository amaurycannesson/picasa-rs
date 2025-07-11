import { isMatch, Link, useMatches } from '@tanstack/react-router';
import { Fragment } from 'react/jsx-runtime';

import {
  Breadcrumb as UiBreadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbSeparator,
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

  const hasGallery = matches[matches.length - 1].routeId.endsWith('/gallery');

  return (
    <UiBreadcrumb>
      <BreadcrumbList>
        {items.map(({ label, href }, index) => (
          <Fragment key={href}>
            <BreadcrumbItem className="hidden md:block">
              <BreadcrumbLink asChild={true}>
                <Link to={index === items.length - 1 && hasGallery ? `${href}/gallery` : href}>
                  {label}
                </Link>
              </BreadcrumbLink>
            </BreadcrumbItem>
            {index < items.length - 1 && <BreadcrumbSeparator />}
          </Fragment>
        ))}
      </BreadcrumbList>
    </UiBreadcrumb>
  );
};

export { Breadcrumb };
