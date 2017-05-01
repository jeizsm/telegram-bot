use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendContact<'c, 'p, 'f, 'l> {
    chat_id: ChatId<'c>,
    phone_number: Cow<'p, str>,
    first_name: Cow<'f, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Cow<'l, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 'p, 'f, 'l> Request for SendContact<'c, 'p, 'f, 'l> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "sendContact"
    }
}

impl<'c, 'p, 'f, 'l> SendContact<'c, 'p, 'f, 'l> {
    pub fn new<C, P, F>(chat: C, phone_number: P, first_name: F) -> Self
        where C: Into<ChatId<'c>>,
              P: Into<Cow<'p, str>>,
              F: Into<Cow<'f, str>>
    {
        SendContact {
            chat_id: chat.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn last_name<F>(mut self, last_name: F) -> Self where F: Into<Cow<'l, str>> {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn disable_notification(mut self) -> Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(mut self, to: R) -> Self where R: Into<MessageId> {
        self.reply_to_message_id = Some(to.into().0);
        self
    }

    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self where R: Into<ReplyMarkup> {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}