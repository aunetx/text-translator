use crate::*;

#[derive(Debug)]
pub enum Engine<'a> {
    TranslateShell(translate_shell::Translator),
    Api(api::Translator<'a>),
}

impl<'a> Engine<'a> {
    pub fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
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
