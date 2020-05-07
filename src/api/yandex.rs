use crate::*;

// ! to interact with api, use either :
// !
// ! yandex_translate_async => async
// !
// ! yandex_translate => simpler
// !
// ! homemade => more complete, could use one-time api key

pub fn translate(
    _text: String,
    _source_language: LanguageType,
    _target_language: Language,
    _key: &String,
) -> Result<String, crate::Error> {
    panic!()
}
