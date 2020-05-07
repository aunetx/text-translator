use crate::*;

pub mod yandex;

#[derive(Debug)]
pub enum Translator {
    Yandex { key: String },
    Google,
}

impl Translator {
    pub fn translate(
        &self,
        text: String,
        source_language: LanguageType,
        target_language: Language,
    ) -> Result<String, Error> {
        use Translator::*;
        match self {
            Yandex { key } => yandex::translate(text, source_language, target_language, key),
            _ => unimplemented!(),
        }
    }
}
