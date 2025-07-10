import { ImageIcon, ImageOffIcon } from 'lucide-react';
import { useEffect, useState } from 'react';

import { commands } from '@/bindings';
import { Skeleton } from '@/components/ui/skeleton';

const PhotoThumbnail: React.FC<{ photoPath: string }> = ({ photoPath }) => {
  const [imageSrc, setImageSrc] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    const loadImage = async () => {
      setLoading(true);
      setError(false);

      const result = await commands.loadPhoto(photoPath);

      if (result.status === 'ok') {
        const blob = new Blob([new Uint8Array(result.data)], {
          type: 'image/webp',
        });
        const url = URL.createObjectURL(blob);

        setImageSrc(url);
      } else {
        setError(true);
      }

      setLoading(false);
    };

    loadImage();

    return () => {
      if (imageSrc) {
        URL.revokeObjectURL(imageSrc);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [photoPath]);

  if (loading) {
    return (
      <Skeleton className="flex h-full w-full items-center justify-center">
        <ImageIcon className="text-muted-foreground h-8 w-8" />
      </Skeleton>
    );
  }

  if (error) {
    return (
      <div className="flex h-full w-full items-center justify-center bg-gray-50">
        <ImageOffIcon className="text-muted-foreground h-8 w-8" />
      </div>
    );
  }

  return <img src={imageSrc} alt="Photo thumbnail" className="h-full w-full object-cover" />;
};

export { PhotoThumbnail };
