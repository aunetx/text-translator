extern crate futures;
extern crate tokio;
extern crate yandex_translate_async;

use gtk_text_translator::*;

const TRANSLATOR_ENGINE: Engine = Engine::TranslateShell(translate_shell::Translator::Google);
const TO_TRANSLATE: &str = "Hello, my name is Naruto Uzumaki!";
const SOURCE_LANGUAGE: LanguageType = LanguageType::Automatic;
const TARGET_LANGUAGE: Language = Language::French;

fn main() {
    println!("Using translator {:?}", TRANSLATOR_ENGINE);
    println!(
        "Translate from {:?} to {:?}",
        SOURCE_LANGUAGE, TARGET_LANGUAGE
    );
    println!("Source = {:?}", TO_TRANSLATE);

    let res =
        TRANSLATOR_ENGINE.translate(TO_TRANSLATE.to_string(), SOURCE_LANGUAGE, TARGET_LANGUAGE);

    println!("Result = {:?}", res);
}
