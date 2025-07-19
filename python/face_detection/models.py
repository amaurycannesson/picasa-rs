from typing import List, Literal

from pydantic import BaseModel


class DetectFacesRequest(BaseModel):
    image_path: str


class BoundingBox(BaseModel):
    x: int
    y: int
    width: int
    height: int


class Face(BaseModel):
    confidence: float
    embedding: List[float]
    bbox: BoundingBox
    gender: Literal["male", "female"]


class DetectFacesResponse(BaseModel):
    faces: List[Face]
