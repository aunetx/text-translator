/*!
A module containing the implementation of the [Yandex Translate API](https://tech.yandex.com/translate/doc/dg/concepts/about-docpage).

To use it, see the [`Yandex struct`](struct.Yandex.html).
*/

use http::uri::Uri;
use hyper::{body::to_bytes, client::Client};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::runtime::Runtime;
use urlencoding::encode;

use super::*;

/// Base URL used to access the Yandex API.
pub const BASE_URL: &'static str = "https://translate.yandex.net/api/v1.5/tr.json/";

/// # Yandex Translate API
///
/// A struct representing the [Yandex Translate API](https://tech.yandex.com/translate/doc/dg/concepts/about-docpage).
///
/// This API needs a key, which can be provided at [this page](https://translate.yandex.com/developers/keys).
///
/// It implements:
///
/// - language translation, with the default [`Api`](../trait.Api.html) trait
/// - language detection, with the [`ApiDetect`](../trait.ApiDetect.html) trait
/// - API key, with the [`ApiKey`](../trait.ApiDetect.html) trait
///
/// To use it, first construct the struct with a defined API key, then do the desired function calls.
///
/// ## Examples
///
/// *__Important:__ In order to use those examples, you need to get a free API Key on the
/// [Yandex website](https://translate.yandex.com/developers/keys) and replace the `YANDEX_API_KEY` const with it.*
///
/// ### Text translation
///
/// Translate a text from an unknown language to Japanese:
///
/// ```
/// use text_translator::*;
///
/// // set your personnal API key
/// const YANDEX_API_KEY: &str = "trnsl.1.1.20200507T202428Z.5e03932d06f63e6a.6ca69498c3b22bff94f6eda9ad8c21b4c3320078";
///
/// // construct the struct
/// let translator: Yandex = Yandex::with_key(YANDEX_API_KEY);
///
/// let text: String = "Hello, my name is Naruto Uzumaki!".to_string();
///
/// // translate the text, returns a `Result<String, Error>`
/// let translated_text: String = match translator.translate(text, InputLanguage::Automatic, Language::Japanese) {
///     Ok(result) => result,
///     Err(err) => panic!("API error, could not translate text : {:#?}", err)
/// };
///
/// assert_eq!(translated_text, "こんにちは、鳴門のうずまき!")
/// ```
///
/// ### Language detection
///
/// Detect the language of a text:
///
/// ```
/// use text_translator::*;
///
/// const YANDEX_API_KEY: &str = "trnsl.1.1.20200507T202428Z.5e03932d06f63e6a.6ca69498c3b22bff94f6eda9ad8c21b4c3320078";
///
/// let translator: Yandex = Yandex::with_key(YANDEX_API_KEY);
/// let text: String = "Bonjour, je m'appelle Naruto Uzumaki!".to_string();
///
/// // detect the language, returns a `Result<Option<Language>, Error>`
/// let detected_language: Language = match translator.detect(text) {
///     Ok(response) => match response {
///         Some(language) => language,
///         None => panic!("Could detect language : unknown language"),
///     },
///     Err(err) => panic!("API error, could not detect language : {:#?}", err)
/// };
///
/// assert_eq!(detected_language, Language::French)
/// ```
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Yandex<'a> {
    key: Option<&'a str>,
}

impl<'a> Yandex<'a> {
    /// Returns a new [`Yandex`](struct.Yandex.html) struct with the given API key.
    ///
    /// Can be used in constant definitions.
    pub const fn with_key(key: &'a str) -> Self {
        Self { key: Some(key) }
    }
}

impl<'a> ApiKey<'a> for Yandex<'a> {
    fn set_set(&mut self, key: &'a str) {
        self.key = Some(key)
    }

    fn get_key(&self) -> Option<&'a str> {
        self.key
    }
}

impl<'a> Api for Yandex<'a> {
    /// Returns a new [`Yandex`](struct.Yandex.html) struct without API key.
    ///
    /// To set it, use [`with_key`](struct.Yandex.html#method.with_key) or [`set_key`](../trait.ApiKey.html#tymethod.set_set) methods instead.
    fn new() -> Self {
        Self { key: None }
    }

    // TODO make `translate` async
    fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error> {
        // get translation direction
        let translation_languages = match source_language {
            InputLanguage::Automatic => format!("{}", target_language.to_language_code()),
            InputLanguage::Defined(source) => {
                // verify that source languages != target language
                if source == target_language {
                    return Err(Error::SameLanguages(source, target_language));
                } else {
                    format!(
                        "{}-{}",
                        source.to_language_code(),
                        target_language.to_language_code()
                    )
                }
            }
        };

        // build query
        let mut query: String = String::from(BASE_URL);
        query = format!(
            "{}translate?key={}&lang={}&text={}",
            query,
            match self.key {
                Some(key) => key,
                None => return Err(Error::NoApiKeySet),
            },
            translation_languages,
            encode(text.as_str())
        );

        let mut runtime = match Runtime::new() {
            Ok(res) => res,
            Err(_) => return Err(Error::FailedToCreateTokioRuntime),
        };

        let uri = match query.parse::<Uri>() {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotParseUri(query)),
        };

        let body = runtime.block_on(get_response(uri))?;

        let json_body: TranslateResponse = match from_str(body.as_str()) {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotDerializeJson),
        };

        Ok(json_body.get_text())
    }
}

impl<'a> ApiDetect for Yandex<'a> {
    // TODO make `detect` async
    fn detect(&self, text: String) -> Result<Option<Language>, Error> {
        // build query
        let mut query: String = String::from(BASE_URL);
        query = format!(
            "{}detect?key={}&text={}",
            query,
            match self.key {
                Some(key) => key,
                None => return Err(Error::NoApiKeySet),
            },
            encode(text.as_str())
        );

        let mut runtime = match Runtime::new() {
            Ok(res) => res,
            Err(_) => return Err(Error::FailedToCreateTokioRuntime),
        };

        let uri = match query.parse::<Uri>() {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotParseUri(query)),
        };

        let body = runtime.block_on(get_response(uri))?;

        let json_body: DetectResponse = match from_str(body.as_str()) {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotDerializeJson),
        };

        Ok(json_body.get_lang())
    }
}

/// Returns the response json body, needed to be deserialized.
async fn get_response(uri: Uri) -> Result<String, Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(uri).await.unwrap();

    match res.status().as_u16() {
        200 => (),
        error => return Err(Error::YandexAPIError(YandexError::from_error_code(error))),
    };

    let body = to_bytes(res.into_body()).await.unwrap();
    match std::str::from_utf8(&body) {
        Ok(res) => Ok(res.to_string()),
        Err(err) => Err(Error::CouldNotConvertToUtf8Str(err)),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslateResponse {
    code: u16,
    lang: String,
    text: Vec<String>,
}

impl ApiTranslateResponse for TranslateResponse {
    fn get_text(&self) -> String {
        self.text.join("\n")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DetectResponse {
    code: u16,
    lang: String,
}

impl ApiDetectResponse for DetectResponse {
    fn get_lang(&self) -> Option<Language> {
        Language::from_language_code(self.lang.clone())
    }
}

/// Enum containing different errors that may be returned by the Yandex API.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum YandexError {
    InvalidAPIKey,
    BlockedAPIKey,
    DailyLimitExceeded,
    MaxTextSizeExceeded,
    CouldNotTranslate,
    TranslationDirectionNotSupported,
    UnknownErrorCode(u16),
}

impl ApiError for YandexError {
    fn from_error_code(code: u16) -> Self {
        use YandexError::*;
        match code - 1 {
            401 => InvalidAPIKey,
            402 => BlockedAPIKey,
            404 => DailyLimitExceeded,
            413 => MaxTextSizeExceeded,
            422 => CouldNotTranslate,
            501 => TranslationDirectionNotSupported,
            other => UnknownErrorCode(other),
        }
    }

    fn to_error_code(&self) -> u16 {
        use YandexError::*;
        match self {
            InvalidAPIKey => 401,
            BlockedAPIKey => 402,
            DailyLimitExceeded => 404,
            MaxTextSizeExceeded => 413,
            CouldNotTranslate => 422,
            TranslationDirectionNotSupported => 501,
            UnknownErrorCode(other) => *other,
        }
    }
}

impl std::fmt::Display for YandexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error : {}", &self)
    }
}

impl std::error::Error for YandexError {}
