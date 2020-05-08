use gtk_text_translator::*;

const YANDEX_API_KEY: &str = "hidden";

const TRANSLATOR_ENGINE: Engine = Engine::Api(api::Translator::Yandex {
    key: YANDEX_API_KEY,
});

const TO_TRANSLATE: &str = "Hello, my name is Naruto Uzumaki!";
const SOURCE_LANGUAGE: InputLanguage = InputLanguage::Automatic;
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
