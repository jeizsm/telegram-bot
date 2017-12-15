use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Point {
    #[serde(rename = "forehead")]
    Forehead,
    #[serde(rename = "eyes")]
    Eyes,
    #[serde(rename = "mouth")]
    Mouth,
    #[serde(rename = "chin")]
    Chin,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MaskPosition {
    point: Point,
    x_shift: Float,
    y_shift: Float,
    scale: Float,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum PngSticker {
    FileId(FileRef),
}
