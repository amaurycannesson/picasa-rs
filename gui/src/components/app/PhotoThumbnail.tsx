import { invoke } from '@tauri-apps/api/core';
import { Image } from 'lucide-react';
import { useEffect, useState } from 'react';

import { Skeleton } from '@/components/ui/skeleton';

const PhotoThumbnail: React.FC<{ photoPath: string }> = ({ photoPath }) => {
  const [imageSrc, setImageSrc] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    const loadImage = async () => {
      try {
        setLoading(true);
        setError(false);

        // Get image data from Rust (returns Vec<u8>)
        const imageData: number[] = await invoke('load_photo', {
          path: photoPath,
        });

        // Convert to Blob and create object URL
        const blob = new Blob([new Uint8Array(imageData)], {
          type: 'image/webp',
        });
        const url = URL.createObjectURL(blob);

        setImageSrc(url);
        setLoading(false);
      } catch (err) {
        console.error('Failed to load image:', err);
        setError(true);
        setLoading(false);
      }
    };

    loadImage();

    // Cleanup: revoke object URL when component unmounts
    return () => {
      if (imageSrc) {
        URL.revokeObjectURL(imageSrc);
      }
    };
  }, [photoPath]);

  if (loading) {
    return (
      <Skeleton className="w-full h-full flex items-center justify-center">
        <Image className="h-8 w-8 text-muted-foreground" />
      </Skeleton>
    );
  }

  if (error) {
    return (
      <div className="w-full h-full bg-red-100 flex items-center justify-center">
        Error loading image
      </div>
    );
  }

  return <img src={imageSrc} alt="Photo thumbnail" className="w-full h-full object-cover" />;
};

export { PhotoThumbnail };
