import { FaceWithPerson } from '@/bindings';

interface FaceOverlayProps {
  face: FaceWithPerson;
  scale: number;
  offsetX: number;
  offsetY: number;
}

const FaceOverlay: React.FC<FaceOverlayProps> = ({
  face: { face, person },
  scale,
  offsetX,
  offsetY,
}) => {
  return (
    <div
      className="absolute rounded-sm border-1 border-gray-50 hover:bg-white/20"
      style={{
        left: `${offsetX + face.bbox_x * scale}px`,
        top: `${offsetY + face.bbox_y * scale}px`,
        width: `${face.bbox_width * scale}px`,
        height: `${face.bbox_height * scale}px`,
      }}
    >
      {person?.name && (
        <div className="absolute right-0 bottom-0 rounded-tl bg-black/70 px-1 py-0.5 text-xs text-white">
          {person.name}
        </div>
      )}
    </div>
  );
};

interface FacesOverlayProps {
  faces: FaceWithPerson[];
  imageDisplayDimensions: {
    width: number;
    height: number;
    offsetX: number;
    offsetY: number;
  };
  imageNaturalDimensions: {
    width: number;
    height: number;
  };
}

const FacesOverlay: React.FC<FacesOverlayProps> = ({
  faces,
  imageDisplayDimensions,
  imageNaturalDimensions,
}) => {
  const scale = imageDisplayDimensions.width / imageNaturalDimensions.width;

  return (
    <div className="absolute inset-0">
      {faces.map((face, index) => (
        <FaceOverlay
          key={index}
          face={face}
          scale={scale}
          offsetX={imageDisplayDimensions.offsetX}
          offsetY={imageDisplayDimensions.offsetY}
        />
      ))}
    </div>
  );
};

export { FacesOverlay };
