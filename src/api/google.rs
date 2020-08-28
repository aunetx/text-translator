/*!
A module containing the implementation of the [Google Translate API](https://cloud.google.com/translate/docs).

To use it, see the [`Google struct`](struct.GoogleV2.html).
*/

use http::{uri::Uri, Request};
use hyper::{body::to_bytes, client::Client, Body};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::runtime::Runtime;
use urlencoding::encode;

use super::*;

/// Base URL used to access the Google API.
pub const GOOGLE_V2_BASE_URL: &str = "https://translation.googleapis.com/language/translate/v2";

/// Helper structure of the request boy of a google translate request
#[derive(Serialize)]
struct GoogleRequestBody<'a> {
    q: &'a str,
    source: &'a str,
    target: &'a str,
    format: &'static str,
}

impl<'a> GoogleRequestBody<'a> {
    fn new(q: &'a str, source: &'a str, target: &'a str) -> Self {
        Self {
            q,
            source,
            target,
            format: "text",
        }
    }
}

/// # Google Translate API
///
/// A struct representing the [Google Translate API](https://cloud.google.com/translate/docs/basic/quickstart).
///
/// This API needs a key, which can be provided at [this page](https://cloud.google.com/translate/docs/setup).
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
/// [Google website](https://cloud.google.com/translate/docs/setup) and replace the `GOOGLE_API_KEY` const with it.*
///
/// ### Text translation
///
/// Translate a text from an unknown language to Japanese:
///
/// ```
/// use text_translator::*;
///
/// // set your personnal API key
/// const GOOGLE_API_KEY: &str = "trnsl.1.1.20200507T202428Z.5e03932d06f63e6a.6ca69498c3b22bff94f6eda9ad8c21b4c3320078";
///
/// // construct the struct
/// let translator = GoogleV2::with_key(GOOGLE_API_KEY);
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
/// const GOOGLE_API_KEY: &str = "trnsl.1.1.20200507T202428Z.5e03932d06f63e6a.6ca69498c3b22bff94f6eda9ad8c21b4c3320078";
///
/// let translator = GoogleV2::with_key(GOOGLE_API_KEY);
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
pub struct GoogleV2<'a> {
    key: Option<&'a str>,
}

impl<'a> GoogleV2<'a> {
    /// Returns a new [`Google`](struct.Google.html) struct with the given API key.
    ///
    /// Can be used in constant definitions.
    pub const fn with_key(key: &'a str) -> Self {
        Self { key: Some(key) }
    }
}

impl<'a> ApiKey<'a> for GoogleV2<'a> {
    fn set_set(&mut self, key: &'a str) {
        self.key = Some(key)
    }

    fn get_key(&self) -> Option<&'a str> {
        self.key
    }
}

impl<'a> Api for GoogleV2<'a> {
    /// Returns a new [`Google`](struct.Google.html) struct without API key.
    ///
    /// To set it, use [`with_key`](struct.Google.html#method.with_key) or [`set_key`](../trait.ApiKey.html#tymethod.set_set) methods instead.
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
        let source_language = match source_language {
            InputLanguage::Automatic => {
                return Err(Error::UnknownLanguageCode(String::from("Not implemented.")))
            }
            InputLanguage::Defined(source) => {
                // verify that source languages != target language
                if source == target_language {
                    return Err(Error::SameLanguages(source, target_language));
                }

                source.to_language_code()
            }
        };

        // build query
        let url: String = format!("{}?key={}", GOOGLE_V2_BASE_URL, self.key.unwrap());
        let body = serde_json::to_string(&GoogleRequestBody::new(
            &text,
            source_language,
            target_language.to_language_code(),
        ))
        .unwrap();

        let mut runtime = match Runtime::new() {
            Ok(res) => res,
            Err(_) => return Err(Error::FailedToCreateTokioRuntime),
        };

        let uri = match url.parse::<Uri>() {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotParseUri(url)),
        };

        let body = runtime.block_on(get_response(uri, body))?;

        let json_body: TranslateResponse = match from_str(body.as_str()) {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotDerializeJson),
        };

        Ok(json_body.get_text())
    }
}

impl<'a> ApiDetect for GoogleV2<'a> {
    // TODO make `detect` async
    fn detect(&self, text: String) -> Result<Option<Language>, Error> {
        // build query
        let mut query: String = String::from(GOOGLE_V2_BASE_URL);
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

        let body = runtime.block_on(get_response(uri, String::new()))?;

        let json_body: DetectResponse = match from_str(body.as_str()) {
            Ok(res) => res,
            Err(_) => return Err(super::Error::CouldNotDerializeJson),
        };

        Ok(json_body.get_lang())
    }
}

/// Returns the response json body, needed to be deserialized.
async fn get_response(uri: Uri, body: String) -> Result<String, Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .body(Body::from(body))
        .expect("request builder");

    let res = client.request(req).await.unwrap();

    match res.status().as_u16() {
        200 => (),
        error => return Err(Error::GoogleAPIError(GoogleError::from_error_code(error))),
    };

    let body = to_bytes(res.into_body()).await.unwrap();
    match std::str::from_utf8(&body) {
        Ok(res) => Ok(res.to_string()),
        Err(err) => Err(Error::CouldNotConvertToUtf8Str(err)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponse {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    translations: Vec<Translation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

impl ApiTranslateResponse for TranslateResponse {
    fn get_text(&self) -> String {
        self.data
            .translations
            .iter()
            .map(|translation| &translation.translated_text[..])
            .collect::<Vec<&str>>()
            .join("\n")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DetectResponse {
    code: u16,
    lang: String,
}

impl ApiDetectResponse for DetectResponse {
    fn get_lang(&self) -> Option<Language> {
        Language::from_language_code(&self.lang)
    }
}

/// Enum containing different errors that may be returned by the Google API.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum GoogleError {
    InvalidAPIKey,
    BlockedAPIKey,
    DailyLimitExceeded,
    MaxTextSizeExceeded,
    CouldNotTranslate,
    TranslationDirectionNotSupported,
    UnknownErrorCode(u16),
}

impl ApiError for GoogleError {
    fn from_error_code(code: u16) -> Self {
        match code - 1 {
            401 => GoogleError::InvalidAPIKey,
            402 => GoogleError::BlockedAPIKey,
            404 => GoogleError::DailyLimitExceeded,
            413 => GoogleError::MaxTextSizeExceeded,
            422 => GoogleError::CouldNotTranslate,
            501 => GoogleError::TranslationDirectionNotSupported,
            other => GoogleError::UnknownErrorCode(other),
        }
    }

    fn to_error_code(&self) -> u16 {
        match self {
            GoogleError::InvalidAPIKey => 401,
            GoogleError::BlockedAPIKey => 402,
            GoogleError::DailyLimitExceeded => 404,
            GoogleError::MaxTextSizeExceeded => 413,
            GoogleError::CouldNotTranslate => 422,
            GoogleError::TranslationDirectionNotSupported => 501,
            GoogleError::UnknownErrorCode(other) => *other,
        }
    }
}

impl std::fmt::Display for GoogleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error : {}", &self)
    }
}

impl std::error::Error for GoogleError {}
