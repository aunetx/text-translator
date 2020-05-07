use crate::*;

#[derive(Debug)]
pub enum Engine {
    TranslateShell(translate_shell::Translator),
    Api(api::Translator),
}

impl Engine {
    pub fn translate(
        &self,
        text: String,
        source_language: LanguageType,
        target_language: Language,
    ) -> Result<String, Error> {
        match self {
            Engine::TranslateShell(translator) => {
                translator.translate(text, source_language, target_language)
            }
            Engine::Api(translator) => translator.translate(text, source_language, target_language),
        }
    }
}
