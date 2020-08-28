/*!
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
- `[ ]` [Google Translate](https://cloud.google.com/translate/docs/)
- `[ ]` [Bing](https://azure.microsoft.com/en-us/services/cognitive-services/translator-text-api/)

## How to use

To use it, you first need to construct a translator (a struct implementing the [Api](trait.Api.html) trait).

Then, you will be able to do various function calls on this struct:

- [`my_translator.translate(my_text, input_language, target_language)`](trait.Api.html#tymethod.translate)
- [`my_translator.detect(my_text)`](trait.ApiDetect.html#tymethod.detect) if the API implements language detection

Languages are represented with the [`Language`](enum.Language.html) enum for target language, and [`InputLanguage`](enum.InputLanguage.html) for input language.
See their respective documentations for more.

## Examples

For the moment, only the Yandex API is implemented.

To see examples on how to use it, see [its documentation](struct.Yandex.html).
*/

mod api;
mod languages;

pub use api::*;
pub use languages::*;

/// Enum containing different errors that may be raised by the program at runtime.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    /// Error when trying to convert translation result to utf-8.
    CouldNotConvertToUtf8String(std::string::FromUtf8Error),
    /// Error when trying to convert translation result to utf-8.
    CouldNotConvertToUtf8Str(std::str::Utf8Error),
    /// Error when deserializing JSON string.
    CouldNotDerializeJson,
    /// Error when sending API request : no KEY set.
    NoApiKeySet,
    /// Error parsing query to a valid URI.
    CouldNotParseUri(String),
    /// Error executing `tokio::runtime::Runtime::new()`.
    FailedToCreateTokioRuntime,
    /// Language input and output are the same.
    SameLanguages(Language, Language),
    /// Could not retrieve language code.
    UnknownLanguageCode(String),
    /// Yandex API error.
    YandexAPIError(api::yandex::YandexError),
    GoogleV2APIError(api::google_v2::GoogleV2Error),
    GoogleV3APIError(api::google_v3::GoogleV3Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error : {}", &self)
    }
}

impl std::error::Error for Error {}
