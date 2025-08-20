use std::string::FromUtf8Error;

use eyre::Result;
// Cast is available in gtk4::prelude::*
use gtk4::prelude::*;

pub trait StringExt {
    fn remove_null_bytes(self) -> Result<String, FromUtf8Error>;
}

impl StringExt for String {
    fn remove_null_bytes(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(
            self.into_bytes()
                .into_iter()
                .filter(|b| *b != 0)
                .collect::<Vec<u8>>(),
        )
    }
}

pub trait SourceViewExt {
    fn get_sourceview_buffer(&self) -> Result<sourceview5::Buffer>;
}

impl SourceViewExt for sourceview5::View {
    fn get_sourceview_buffer(&self) -> Result<sourceview5::Buffer> {
        self.buffer()
            .downcast::<sourceview5::Buffer>()
            .map_err(|_| eyre!("Failed to downcast TextBuffer to sourceview Buffer"))
    }
}
