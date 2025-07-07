import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';

import { commands, PaginatedPhotos } from '@/bindings';
import { PhotoThumbnail } from '@/components/app/PhotoThumbnail';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination';
import { useSidebar } from '@/components/ui/sidebar';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const [paginatedPhotos, setPaginatedPhotos] = useState<PaginatedPhotos | null>(null);
  const [currentPage, setCurrentPage] = useState(1);
  const [perPage] = useState(20);
  const [searchText, setSearchText] = useState('');
  const [searchCountry, setSearchCountry] = useState('');
  const [searchCity, setSearchCity] = useState('');
  const { open, isMobile } = useSidebar();

  async function search(page: number = 1) {
    const params = {
      text: searchText || null,
      threshold: null,
      country: searchCountry || null,
      city: searchCity || null,
      date_from: null,
      date_to: null,
      page,
      per_page: perPage,
    };

    const result = await commands.searchPhotos(params);

    if (result.status === 'ok') {
      setPaginatedPhotos(result.data);
      setCurrentPage(page);
    }
  }
  return (
    <div>
      <div className="flex gap-2 mb-4">
        <Input
          placeholder="Search..."
          value={searchText}
          onChange={(e) => setSearchText(e.target.value)}
          className="flex-1"
        />
        <Input
          placeholder="Country..."
          value={searchCountry}
          onChange={(e) => setSearchCountry(e.target.value)}
          className="flex-1"
        />
        <Input
          placeholder="City..."
          value={searchCity}
          onChange={(e) => setSearchCity(e.target.value)}
          className="flex-1"
        />
        <Button onClick={() => search(1)}>Search</Button>
      </div>

      {paginatedPhotos && paginatedPhotos.items.length > 0 ? (
        <>
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-4">
            {paginatedPhotos.items.map((p) => (
              <div
                key={p.path}
                className="aspect-square overflow-hidden rounded-lg bg-white shadow-sm hover:shadow-md transition-shadow"
              >
                <PhotoThumbnail photoPath={p.path} />
              </div>
            ))}
          </div>

          {paginatedPhotos.total_pages > 1 && (
            <div
              className="fixed bottom-4 left-1/2 transform -translate-x-1/2  rounded-lg shadow-lg border p-2 z-10"
              style={{
                marginLeft: open && !isMobile ? '7rem' : '0',
              }}
            >
              <Pagination>
                <PaginationContent>
                  <PaginationItem>
                    <PaginationPrevious
                      onClick={() => currentPage > 1 && search(currentPage - 1)}
                      className={
                        currentPage <= 1 ? 'pointer-events-none opacity-50' : 'cursor-pointer'
                      }
                    />
                  </PaginationItem>

                  {Array.from({ length: paginatedPhotos.total_pages }, (_, i) => i + 1).map(
                    (page) => (
                      <PaginationItem key={page}>
                        <PaginationLink
                          onClick={() => search(page)}
                          isActive={currentPage === page}
                          className="cursor-pointer"
                        >
                          {page}
                        </PaginationLink>
                      </PaginationItem>
                    ),
                  )}

                  <PaginationItem>
                    <PaginationNext
                      onClick={() =>
                        currentPage < paginatedPhotos.total_pages && search(currentPage + 1)
                      }
                      className={
                        currentPage >= paginatedPhotos.total_pages
                          ? 'pointer-events-none opacity-50'
                          : 'cursor-pointer'
                      }
                    />
                  </PaginationItem>
                </PaginationContent>
              </Pagination>
            </div>
          )}
        </>
      ) : (
        <p className="text-gray-500">No photos.</p>
      )}
    </div>
  );
}
