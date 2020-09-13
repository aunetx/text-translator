# text_translator

## Description

This crate permits to translate text between languages easily. Its goals are:

- implementing an unique library for different APIs
- permitting language translations / detections with or withtout API key when possible
- ease of use / relative performances
- (later) async translations

It wants to implement the following APIs:

- `[x]` [Yandex.Translate](https://tech.yandex.com/translate/doc/dg/concepts/about-docpage)
  - `[x]` with [API key](https://translate.yandex.com/developers/keys)
  - `[ ]` without key (5_000 chars/translation max)
- `[x]` [Google Translate](https://cloud.google.com/translate/docs/)
- `[ ]` [Bing](https://azure.microsoft.com/en-us/services/cognitive-services/translator-text-api/)

## How to use

To use it, you first need to construct a translator (a struct implementing the [Api](https://docs.rs/text-translator/latest/text_translator/trait.Api.html) trait).

Then, you will be able to do various function calls on this struct:

- [`my_translator.translate(my_text, input_language, target_language)`](https://docs.rs/text-translator/latest/text_translator/trait.Api.html#tymethod.translate)
- [`my_translator.detect(my_text)`](https://docs.rs/text-translator/latest/text_translator/trait.ApiDetect.html#tymethod.detect) if the API implements language detection

Languages are represented with the [`Language`](https://docs.rs/text-translator/latest/text_translator/enum.Language.html) enum for target language, and [`InputLanguage`](https://docs.rs/text-translator/latest/text_translator/enum.InputLanguage.html) for input language.
See their respective documentations for more.

### Examples

For the moment, [only the Google API is working](https://docs.rs/text-translator/latest/text_translator/struct.GoogleV2.html).

There was a change in the Yandex API, the current [implementation](https://docs.rs/text-translator/latest/text_translator/struct.Yandex.html) is not compatible.

Those are examples on how to use it to translate a text, and to detect the input language.

*__Important:__ In order to use those examples, you need to get a free API Key on the [Google website](https://cloud.google.com/translate/docs/setup)*.

#### Text translation

Translate a text from an unknown language to Japanese:

```rust
use text_translator::*;

// replace with your personnal API key
const YANDEX_API_KEY: &str = "MY_PRIVATE_KEY_SET_YOUR_OWN";

// construct the struct
let translator: Yandex = Yandex::with_key(YANDEX_API_KEY);

let text: String = "Hello, my name is Naruto Uzumaki!".to_string();

// translate the text, returns a `Result<String, Error>`
let translated_text: String = match translator.translate(text, InputLanguage::Automatic, Language::Japanese) {
    Ok(result) => result,
    Err(err) => panic!("API error, could not translate text : {:#?}", err)
};

assert_eq!(translated_text, "こんにちは、鳴門のうずまき!")
```

#### Language detection

Detect the language of a text:

```rust
use text_translator::*;

// replace with your personnal API key
const YANDEX_API_KEY: &str = "MY_PRIVATE_KEY_SET_YOUR_OWN";

let translator: Yandex = Yandex::with_key(YANDEX_API_KEY);
let text: String = "Bonjour, je m'appelle Naruto Uzumaki!".to_string();

// detect the language, returns a `Result<Option<Language>, Error>`
let detected_language: Language = match translator.detect(text) {
    Ok(response) => match response {
        Some(language) => language,
        None => panic!("Could detect language : unknown language"),
    },
    Err(err) => panic!("API error, could not detect language : {:#?}", err)
};

assert_eq!(detected_language, Language::French)
```

## Contributing

All contributions are welcome!

If you find any issue, please report it — if you have recommendations too!
