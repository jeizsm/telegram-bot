use types::*;
use requests::*;

/// Use this method to get a sticker set.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetStickerSet {
    name: StickerSetName,
}

impl Request for GetStickerSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<StickerSet>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getStickerSet"), self)
    }
}

impl GetStickerSet {
    pub fn new(name: StickerSetName) -> Self {
        Self {
            name: name
        }
    }
}
