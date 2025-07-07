import { useMatches } from '@tanstack/react-router';

import {
  Breadcrumb as UiBreadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
} from '../ui/breadcrumb';

const Breadcrumb = () => {
  const matches = useMatches();

  const breadcrumb = (matches[1].staticData as { [k: string]: string }).breadcrumb;

  if (!breadcrumb) return <></>;

  return (
    <UiBreadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem className="hidden md:block">
          <BreadcrumbLink href="#">{breadcrumb}</BreadcrumbLink>
        </BreadcrumbItem>
      </BreadcrumbList>
    </UiBreadcrumb>
  );
};

export { Breadcrumb };
