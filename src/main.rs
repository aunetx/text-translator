use gtk_text_translator::*;

const _YANDEX_API_KEY: &str = "hidden";

const _TRANSLATE_SHELL: Engine = Engine::TranslateShell(translate_shell::Translator::Google);
const _TRANSLATE_API: Engine = Engine::Api(api::Translator::Yandex {
    key: _YANDEX_API_KEY,
});

const TO_TRANSLATE: &str = "Hello, my name is Naruto Uzumaki!\nI love noodles and fights.";
const SOURCE_LANGUAGE: InputLanguage = InputLanguage::Automatic;
const TARGET_LANGUAGE: Language = Language::Kannada;

fn main() {
    let res =
        _TRANSLATE_SHELL.translate(TO_TRANSLATE.to_string(), SOURCE_LANGUAGE, TARGET_LANGUAGE);

    println!("\nTranslated text = \n\n{}", res.unwrap())
}

// TODO create tests for:
// - all languages
// - all engines
// - all translators
