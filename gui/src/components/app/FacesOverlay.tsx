import { Face } from '@/bindings';

interface FaceOverlayProps {
  face: Face;
  scale: number;
  offsetX: number;
  offsetY: number;
}

const FaceOverlay: React.FC<FaceOverlayProps> = ({ face, scale, offsetX, offsetY }) => {
  return (
    <div
      className="absolute border-2 border-red-500 bg-red-500/10"
      style={{
        left: `${offsetX + face.bbox_x * scale}px`,
        top: `${offsetY + face.bbox_y * scale}px`,
        width: `${face.bbox_width * scale}px`,
        height: `${face.bbox_height * scale}px`,
      }}
    />
  );
};

interface FacesOverlayProps {
  faces: Face[];
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
      {faces.map((face) => (
        <FaceOverlay
          key={face.id}
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
