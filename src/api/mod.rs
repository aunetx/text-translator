use crate::*;

mod yandex;
pub use yandex::*;

/// A trait defining a translate API.
///
/// Implements `new()` to return a new API, and `translate()` to translate a text.
pub trait Api {
    fn new() -> Self;

    fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error>;
}

/// A trait extending `Api`, where the API is capable of detecting the language of a text.
pub trait ApiDetect: Api {
    fn detect(&self, text: String) -> Result<Option<Language>, Error>;
}

/// A trait extending `Api`, where the API needs to have a API Key.
pub trait ApiKey<'a>: Api + Sized {
    fn set_set(&mut self, key: &'a str);

    fn get_key(&self) -> Option<&'a str>;
}

trait ApiTranslateResponse {
    fn get_text(&self) -> String;
}

trait ApiDetectResponse {
    fn get_lang(&self) -> Option<Language>;
}

trait ApiError {
    fn from_error_code(code: u16) -> Self;
}
