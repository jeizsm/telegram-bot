use futures::{Future};

use errors::Error;

pub type TelegramFuture<T> = Box<Future<Item = T, Error = Error>>;
