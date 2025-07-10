import { Link } from '@tanstack/react-router';

import { PhotoThumbnail } from '@/components/app/PhotoThumbnail';
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination';
import { useSidebar } from '@/components/ui/sidebar';

export interface NavigationConfig {
  to: string;
  params?: Record<string, string | number>;
  search: Record<string, string | number | null>;
}

export interface PhotoGalleryProps {
  photos: Array<{ path: string }>;
  currentPage: number;
  totalPages: number;
  getNavigationConfig: (page: number) => NavigationConfig;
}

export function PhotoGallery({
  photos,
  currentPage,
  totalPages,
  getNavigationConfig,
}: PhotoGalleryProps) {
  const { open, isMobile } = useSidebar();

  if (!photos || photos.length === 0) {
    return <p className="text-gray-500">No photos.</p>;
  }

  return (
    <div>
      <div className="grid grid-cols-2 gap-4 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8">
        {photos.map((p) => (
          <div
            key={p.path}
            className="aspect-square overflow-hidden rounded-lg bg-white shadow-sm transition-shadow hover:shadow-md"
          >
            <PhotoThumbnail photoPath={p.path} />
          </div>
        ))}
      </div>

      {totalPages > 1 && (
        <div
          className="dark:bg-background bg-background fixed bottom-4 left-1/2 z-10 -translate-x-1/2 transform rounded-lg border p-2 shadow-lg"
          style={{
            marginLeft: open && !isMobile ? '7rem' : '0',
          }}
        >
          <Pagination>
            <PaginationContent>
              <PaginationItem>
                {currentPage > 1 ? (
                  <Link {...getNavigationConfig(currentPage - 1)}>
                    <PaginationPrevious />
                  </Link>
                ) : (
                  <PaginationPrevious className="pointer-events-none opacity-50" />
                )}
              </PaginationItem>

              {(() => {
                const pages = [];

                if (totalPages <= 7) {
                  // Show all pages if 7 or fewer
                  for (let i = 1; i <= totalPages; i++) {
                    pages.push(
                      <PaginationItem key={i}>
                        <PaginationLink isActive={currentPage === i} className="cursor-pointer">
                          <Link {...getNavigationConfig(i)}>
                            {i}
                          </Link>
                        </PaginationLink>
                      </PaginationItem>,
                    );
                  }
                } else {
                  // Show first page
                  pages.push(
                    <PaginationItem key={1}>
                      <PaginationLink isActive={currentPage === 1} className="cursor-pointer">
                        <Link {...getNavigationConfig(1)}>
                          1
                        </Link>
                      </PaginationLink>
                    </PaginationItem>,
                  );

                  // Show ellipsis if needed
                  if (currentPage > 3) {
                    pages.push(
                      <PaginationItem key="ellipsis-start">
                        <PaginationEllipsis />
                      </PaginationItem>,
                    );
                  }

                  // Show pages around current page
                  const start = Math.max(2, currentPage - 1);
                  const end = Math.min(totalPages - 1, currentPage + 1);

                  for (let i = start; i <= end; i++) {
                    pages.push(
                      <PaginationItem key={i}>
                        <PaginationLink isActive={currentPage === i} className="cursor-pointer">
                          <Link {...getNavigationConfig(i)}>
                            {i}
                          </Link>
                        </PaginationLink>
                      </PaginationItem>,
                    );
                  }

                  // Show ellipsis if needed
                  if (currentPage < totalPages - 2) {
                    pages.push(
                      <PaginationItem key="ellipsis-end">
                        <PaginationEllipsis />
                      </PaginationItem>,
                    );
                  }

                  // Show last page
                  if (totalPages > 1) {
                    pages.push(
                      <PaginationItem key={totalPages}>
                        <PaginationLink
                          isActive={currentPage === totalPages}
                          className="cursor-pointer"
                        >
                          <Link {...getNavigationConfig(totalPages)}>
                            {totalPages}
                          </Link>
                        </PaginationLink>
                      </PaginationItem>,
                    );
                  }
                }

                return pages;
              })()}

              <PaginationItem>
                {currentPage < totalPages ? (
                  <Link {...getNavigationConfig(currentPage + 1)}>
                    <PaginationNext />
                  </Link>
                ) : (
                  <PaginationNext className="pointer-events-none opacity-50" />
                )}
              </PaginationItem>
            </PaginationContent>
          </Pagination>
        </div>
      )}
    </div>
  );
}