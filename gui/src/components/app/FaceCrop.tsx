import { Image } from 'lucide-react';
import { useEffect, useState } from 'react';

import { commands } from '@/bindings';
import { Skeleton } from '@/components/ui/skeleton';

import { AspectRatio } from '../ui/aspect-ratio';

const FaceCrop: React.FC<{ faceId: number }> = ({ faceId }) => {
  const [imageSrc, setImageSrc] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    const loadImage = async () => {
      try {
        setLoading(true);
        setError(false);

        const result = await commands.loadFaceImage(faceId);

        if (result.status === 'ok') {
          const imageData = result.data;
          const blob = new Blob([new Uint8Array(imageData)], {
            type: 'image/webp',
          });
          const url = URL.createObjectURL(blob);
          setImageSrc(url);
        }

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
  }, [faceId]);

  if (loading || error) {
    return (
      <AspectRatio ratio={1}>
        <Skeleton className="w-full h-full flex items-center justify-center">
          <Image className="h-8 w-8 text-muted-foreground" />
        </Skeleton>
      </AspectRatio>
    );
  }

  return (
    <AspectRatio ratio={1}>
      <img src={imageSrc} alt="Face" className="h-full w-full rounded-md " />;
    </AspectRatio>
  );
};

export { FaceCrop };
