use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct EditMessageReplyMarkup<'c> {
    pub chat_id: ChatId<'c>,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'c> Request for EditMessageReplyMarkup<'c> {
    type Response = Message;

    fn name(&self) -> &'static str {
        "editMessageReplyMarkup"
    }
}

impl<'c> EditMessageReplyMarkup<'c> {
    pub fn new<C, M, R>(chat: C, message_id: M, reply_markup: Option<R>) -> Self
        where C: Into<ChatId<'c>>, M: Into<MessageId>, R: Into<ReplyMarkup> {

        EditMessageReplyMarkup {
            chat_id: chat.into(),
            message_id: message_id.into(),
            reply_markup: reply_markup.map(|r| r.into()),
        }
    }

}

pub trait CanEditMessageReplyMarkup {
    fn edit_reply_markup<'c, R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup<'c> where R: Into<ReplyMarkup>;
}

impl CanEditMessageReplyMarkup for Message {
    fn edit_reply_markup<'c, R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup<'c> where R: Into<ReplyMarkup> {
        EditMessageReplyMarkup::new(&self.chat, self, reply_markup)
    }
}
