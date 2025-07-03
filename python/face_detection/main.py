from fastapi import FastAPI, HTTPException

from face_detection_service import FaceDetectionService
from models import DetectFacesRequest, DetectFacesResponse


app = FastAPI()

face_detection_service = FaceDetectionService()


@app.post("/detect-faces", response_model=DetectFacesResponse)
async def detect_faces(request: DetectFacesRequest):
    try:
        detected_faces = face_detection_service.detect_faces(request.image_path)
        return DetectFacesResponse(faces=detected_faces)
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Face detection failed: {str(e)}")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
