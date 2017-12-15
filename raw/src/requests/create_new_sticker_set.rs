use types::*;
use requests::*;
use std::borrow::Cow;
use std::ops::Not;

/// Use this method to get a sticker set.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CreateNewStickerSet<'t, 'e> {
    user_id: UserId,
    name: StickerSetName,
    title: Cow<'t, str>,
    png_sticker: FileRef,
    emojis: Cow<'e, str>,
    #[serde(skip_serializing_if = "Not::not")]
    contains_masks: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>
}

impl<'t, 'e> Request for CreateNewStickerSet<'t, 'e> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("createNewStickerSet"), self)
    }
}

impl<'t, 'e> CreateNewStickerSet<'t, 'e> {
    pub fn new<U, T, E, F>(user_id: U,
                        name: StickerSetName,
                        title: T,
                        png_sticker: F,
                        emojis: E,
                        contains_masks: bool,
                        mask_position: Option<MaskPosition>) -> Self
        where U: ToUserId,
              F: ToFileRef,
              T: Into<Cow<'t, str>>,
              E: Into<Cow<'e, str>>
    {
        Self {
            user_id: user_id.to_user_id(),
            name: name,
            title: title.into(),
            png_sticker: png_sticker.to_file_ref(),
            emojis: emojis.into(),
            contains_masks: contains_masks,
            mask_position: mask_position
        }
    }
}
