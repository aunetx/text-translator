use super::{Api, ApiKey};
use crate::*;

// ! to interact with api, use either :
// !
// ! yandex_translate_async => async
// !
// ! yandex_translate => simpler
// !
// ! homemade => more complete, could use one-time api key

pub struct Yandex<'a> {
    key: Option<&'a str>,
}

impl<'a> ApiKey<'a> for Yandex<'a> {
    fn with_key(key: &'a str) -> Self {
        Self { key: Some(key) }
    }

    fn set_set(&mut self, key: &'a str) {
        self.key = Some(key)
    }

    fn get_key(&self) -> Option<&'a str> {
        self.key.clone()
    }
}

impl<'a> Api for Yandex<'a> {
    fn new() -> Self {
        Self { key: None }
    }

    fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error> {
        unimplemented!()
    }
}
