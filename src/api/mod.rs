use crate::*;

mod yandex;
pub use yandex::*;

#[derive(Debug)]
pub enum Translator<'a> {
    Yandex { key: &'a str },
    Google,
}

impl<'a> Translator<'a> {
    pub fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error> {
        match self {
            Translator::Yandex { key } => {
                Yandex::with_key(key.clone()).translate(text, source_language, target_language)
            }
            _ => unimplemented!(),
        }
    }
}

pub trait Api {
    fn new() -> Self;

    fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error>;
}

pub trait ApiKey<'a>: Api + Sized {
    fn with_key(key: &'a str) -> Self;

    fn set_set(&mut self, key: &'a str);

    fn get_key(&self) -> Option<&'a str>;
}

pub trait ApiResponse {
    fn get_text(&self) -> String;
}

pub trait ApiError {
    fn from_error_code(code: u16) -> Self;
}
