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

/// Get basic info about a file and prepare it for downloading.
pub trait CanGetStickerSet {
    fn get_sticker_set(&self) -> GetStickerSet;
}

impl<F> CanGetStickerSet for F where F: ToStickerSetName {
    fn get_sticker_set(&self) -> GetStickerSet {
        GetStickerSet::new(self.to_sticker_set_name())
    }
}
