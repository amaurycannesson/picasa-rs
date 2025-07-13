import { createFileRoute } from '@tanstack/react-router';

import { commands } from '@/bindings';
import { ErrorMessage } from '@/components/app/ErrorMessage';
import { Photo } from '@/components/app/Photo';

export const Route = createFileRoute('/photo/$id')({
  component: RouteComponent,
  loader: async ({ params: { id } }) => {
    const photo = await commands.getPhoto(parseInt(id));

    if (photo.status === 'error') throw new Error(photo.error);

    return {
      photo: photo.data,
    };
  },
  errorComponent: ErrorMessage,
});

function RouteComponent() {
  const { photo } = Route.useLoaderData();
  return (
    <div className="flex h-[calc(100vh-6rem)]">
      <div className="flex-1/2">
        <Photo photoPath={photo.path} />
      </div>
      <div className="flex-1 overflow-y-auto px-4">
        <div className="grid grid-cols-1 gap-2">
          <div>
            <div className="text-muted-foreground text-sm">File name</div>
            <div>{photo.file_name}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">File path</div>
            <div className="break-all">{photo.path}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">File size</div>
            <div>{(photo.file_size / 1024 / 1024).toFixed(2)} MB</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Hash</div>
            <div className="break-all">{photo.hash || 'N/A'}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Created at</div>
            <div>{new Date(photo.created_at).toLocaleString()}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Modified at</div>
            <div>{new Date(photo.modified_at).toLocaleString()}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Date taken (local)</div>
            <div>
              {photo.date_taken_local ? new Date(photo.date_taken_local).toLocaleString() : 'N/A'}
            </div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Date taken (UTC)</div>
            <div>
              {photo.date_taken_utc ? new Date(photo.date_taken_utc).toLocaleString() : 'N/A'}
            </div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Image dimensions</div>
            <div>
              {photo.image_width && photo.image_height
                ? `${photo.image_width} × ${photo.image_height}`
                : 'N/A'}
            </div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Orientation</div>
            <div>{photo.orientation || 'N/A'}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Camera make</div>
            <div>{photo.camera_make || 'N/A'}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Camera model</div>
            <div>{photo.camera_model || 'N/A'}</div>
          </div>

          <div>
            <div className="text-muted-foreground text-sm">Lens model</div>
            <div>{photo.lens_model || 'N/A'}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
