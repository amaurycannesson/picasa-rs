import { ImageIcon, ImageOffIcon } from 'lucide-react';
import { useCallback, useEffect, useRef, useState } from 'react';

import { commands, Face } from '@/bindings';
import { Skeleton } from '@/components/ui/skeleton';

import { FacesOverlay } from './FacesOverlay';

const Photo: React.FC<{ photoPath: string; asThumbnail?: boolean; faces?: Face[] }> = ({
  photoPath,
  asThumbnail,
  faces = [],
}) => {
  const [imageSrc, setImageSrc] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [imageNaturalDimensions, setImageNaturalDimensions] = useState<{
    width: number;
    height: number;
  } | null>(null);
  const [imageDisplayDimensions, setImageDisplayDimensions] = useState<{
    width: number;
    height: number;
    offsetX: number;
    offsetY: number;
  } | null>(null);
  const imgRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    const loadImage = async () => {
      setLoading(true);
      setError(false);

      const result = asThumbnail
        ? await commands.loadPhotoThumbnail(photoPath)
        : await commands.loadPhoto(photoPath);

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

  const calculateImageDimensions = useCallback(() => {
    if (!imgRef.current || !imageNaturalDimensions) return;

    const img = imgRef.current;
    const imgRect = img.getBoundingClientRect();

    const scaleX = imgRect.width / imageNaturalDimensions.width;
    const scaleY = imgRect.height / imageNaturalDimensions.height;
    const scale = Math.min(scaleX, scaleY); // object-contain behavior

    const displayWidth = imageNaturalDimensions.width * scale;
    const displayHeight = imageNaturalDimensions.height * scale;

    const offsetX = (imgRect.width - displayWidth) / 2;
    const offsetY = (imgRect.height - displayHeight) / 2;

    setImageDisplayDimensions({
      width: displayWidth,
      height: displayHeight,
      offsetX,
      offsetY,
    });
  }, [imageNaturalDimensions]);

  useEffect(() => {
    calculateImageDimensions();

    const handleResize = () => calculateImageDimensions();
    window.addEventListener('resize', handleResize);

    return () => window.removeEventListener('resize', handleResize);
  }, [imageNaturalDimensions, calculateImageDimensions]);

  const handleImageLoad = () => {
    if (imgRef.current) {
      setImageNaturalDimensions({
        width: imgRef.current.naturalWidth,
        height: imgRef.current.naturalHeight,
      });
    }
  };

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

  return (
    <div className="relative h-full w-full">
      <img
        ref={imgRef}
        src={imageSrc}
        alt="Photo thumbnail"
        className={`h-full w-full object-${asThumbnail ? 'cover' : 'contain'}`}
        onLoad={handleImageLoad}
      />
      {!asThumbnail && faces.length > 0 && imageDisplayDimensions && imageNaturalDimensions && (
        <FacesOverlay
          faces={faces}
          imageDisplayDimensions={imageDisplayDimensions}
          imageNaturalDimensions={imageNaturalDimensions}
        />
      )}
    </div>
  );
};

export { Photo };
