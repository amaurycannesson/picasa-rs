from typing import List

import cv2
import numpy as np
import pillow_heif
from insightface.app import FaceAnalysis

from models import BoundingBox, Face


class FaceDetectionService:
    def __init__(self):
        self.app = FaceAnalysis(
            providers=["CPUExecutionProvider"]
        )  # Use 'CUDAExecutionProvider' for GPU
        self.app.prepare(ctx_id=-1)  # ctx_id=-1 for CPU, 0 for GPU

    def detect_faces(self, image_path: str) -> List[Face]:
        img = None
        if image_path.lower().endswith(".heic"):
            heif_file = pillow_heif.open_heif(
                image_path, convert_hdr_to_8bit=False, bgr_mode=True
            )
            img = np.asarray(heif_file[0])
        else:
            img = cv2.imread(image_path)

        if img is None:
            raise ValueError(f"Could not read image: {image_path}")

        faces = self.app.get(img)

        return [
            Face(
                confidence=float(face.det_score),
                embedding=face.normed_embedding.tolist(),
                bbox=BoundingBox(
                    x=int(face.bbox[0]),
                    y=int(face.bbox[1]),
                    width=int(face.bbox[2] - face.bbox[0]),
                    height=int(face.bbox[3] - face.bbox[1]),
                ),
                gender="female" if int(face.gender) else "male",
            )
            for face in faces
        ]
