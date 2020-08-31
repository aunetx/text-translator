/*!
A module containing the implementation of the [Google Translate API](https://cloud.google.com/translate/docs).

To use it, see the [`GoogleV2 struct`](struct.GoogleV2.html).
*/

use async_trait::async_trait;

use http::{uri::Uri, Request};
use hyper::{body::to_bytes, client::Client, Body};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use super::*;

/// Base URL used to access the Google API.
pub const GOOGLE_V2_BASE_URL: &str = "https://translation.googleapis.com/language/translate/v2";

/// Helper structure of the request boy of a google translate request
#[derive(Serialize)]
struct GoogleV2RequestBody<'a> {
    q: &'a str,
    source: Option<&'a str>,
    target: &'a str,
    format: &'static str,
}

impl<'a> GoogleV2RequestBody<'a> {
    fn new(q: &'a str, source: Option<&'a str>, target: &'a str) -> Self {
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
/// This API needs a key, which can be created at [this page](https://cloud.google.com/translate/docs/setup).
///
/// It implements:
///
/// - language translation, with the default [`Api`](../trait.Api.html) trait
/// - language detection, with the [`ApiDetect`](../trait.ApiDetect.html) trait
/// - API key, with the [`ApiKey`](../trait.ApiDetect.html) trait
///
/// To use it, first construct the struct with a defined API key, then do the desired function calls.
///
/// ### Text translation
///
/// Translate a text from an unknown language to Japanese:
///
/// ```
/// use text_translator::*;
///
/// // construct the struct
/// let translator = GoogleV2::with_key("<GOOGLE_API_KEY>");
///
/// let text: String = "There is no real ending. It's just the place where you stop the story.".to_string();
///
/// // translate the text, returns a `Result<String, Error>`
/// let translated_text: String = match translator.translate(text, InputLanguage::Automatic, Language::German) {
///     Ok(result) => result,
///     Err(err) => panic!("API error, could not translate text : {:#?}", err)
/// };
///
/// assert_eq!(translated_text, "Es gibt kein wirkliches Ende. Es ist nur der Ort, an dem Sie die Geschichte stoppen.")
/// ```
///
/// ### Language detection
///
/// Detect the language of a text:
///
/// ```
/// use text_translator::*;
///
/// let translator = GoogleV2::with_key("<GOOGLE_API_KEY>");
/// let text: String = "Es gibt kein wirkliches Ende. Es ist nur der Ort, an dem Sie die Geschichte stoppen.".to_string();
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
/// assert_eq!(detected_language, Language::German)
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

#[async_trait]
impl<'a> Api for GoogleV2<'a> {
    /// Returns a new [`Google`](struct.Google.html) struct without API key.
    ///
    /// To set it, use [`with_key`](struct.Google.html#method.with_key) or [`set_key`](../trait.ApiKey.html#tymethod.set_set) methods instead.
    fn new() -> Self {
        Self { key: None }
    }

    // TODO make `translate` async
    async fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error> {
        // get translation direction
        let source_language = match source_language {
            InputLanguage::Automatic => None,
            InputLanguage::Defined(source) => {
                // verify that source languages != target language
                if source == target_language {
                    return Err(Error::SameLanguages(source, target_language));
                }

                Some(source.to_language_code())
            }
        };

        // build query
        let url: String = format!(
            "{}?key={}",
            GOOGLE_V2_BASE_URL,
            self.key.ok_or(Error::NoApiKeySet)?
        );
        let body = serde_json::to_string(&GoogleV2RequestBody::new(
            &text,
            source_language,
            target_language.to_language_code(),
        ))
        .map_err(|_| Error::CouldNotSerializeJson)?;

        let uri = match url.parse::<Uri>() {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotParseUri(url)),
        };

        let body = get_response(uri, body).await?;

        let json_body: TranslateResponse = match from_str(body.as_str()) {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotDerializeJson),
        };

        Ok(json_body.get_text())
    }
}

#[async_trait]
impl<'a> ApiDetect for GoogleV2<'a> {
    // TODO make `detect` async
    async fn detect(&self, text: String) -> Result<Option<Language>, Error> {
        // build query
        let query = format!(
            "{}/detect?key={}",
            GOOGLE_V2_BASE_URL,
            match self.key {
                Some(key) => key,
                None => return Err(Error::NoApiKeySet),
            },
        );

        let body = format!(r#"{{"q":"{}"}}"#, &text);

        let uri = match query.parse::<Uri>() {
            Ok(res) => res,
            Err(_) => return Err(Error::CouldNotParseUri(query)),
        };

        let body = get_response(uri, body).await?;

        let json_body: GoogleDetectResponse = match from_str(body.as_str()) {
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

    let res = client
        .request(req)
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?;

    match res.status().as_u16() {
        200 => (),
        error => {
            return Err(Error::GoogleV2APIError(GoogleV2Error::from_error_code(
                error,
            )))
        }
    };

    let body = to_bytes(res.into_body())
        .await
        .map_err(|e| Error::RequestError(e.to_string()))?;
    match std::str::from_utf8(&body) {
        Ok(res) => Ok(res.to_string()),
        Err(err) => Err(Error::CouldNotConvertToUtf8Str(err)),
    }
}

/// Serializable struct of a Google transalte response
#[derive(Debug, Serialize, Deserialize)]
struct TranslateResponse {
    data: Data,
}
/// Content of a TranslateResponse
#[derive(Debug, Serialize, Deserialize)]
struct Data {
    translations: Vec<Translation>,
}
/// Text translation in a TranslateResponse
#[derive(Debug, Serialize, Deserialize)]
struct Translation {
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

/// Serializable struct of a Google trasnlate detect request
#[derive(Debug, Serialize, Deserialize)]
struct GoogleDetectResponse {
    data: DetectData,
}

/// Content of a GoogleDetectResponse
#[derive(Debug, Serialize, Deserialize)]
struct DetectData {
    detections: Vec<Vec<Detection>>,
}

/// Text-forat language detected in a GoogleDetectResponse
#[derive(Debug, Serialize, Deserialize)]
struct Detection {
    confidence: i64,
    #[serde(rename = "isReliable")]
    is_reliable: bool,
    language: String,
}

impl ApiDetectResponse for GoogleDetectResponse {
    fn get_lang(&self) -> Option<Language> {
        Language::from_language_code(
            &self
                .data
                .detections
                .iter()
                .map(|detection| detection.first().unwrap())
                .map(|detection| &detection.language[..])
                .collect::<Vec<&str>>()
                .join("\n"),
        )
    }
}

/// Enum containing different errors that may be returned by the Google API.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum GoogleV2Error {
    InvalidAPIKey,
    BlockedAPIKey,
    DailyLimitExceeded,
    MaxTextSizeExceeded,
    CouldNotTranslate,
    TranslationDirectionNotSupported,
    UnknownErrorCode(u16),
}

impl ApiError for GoogleV2Error {
    fn from_error_code(code: u16) -> Self {
        match code - 1 {
            401 => GoogleV2Error::InvalidAPIKey,
            402 => GoogleV2Error::BlockedAPIKey,
            404 => GoogleV2Error::DailyLimitExceeded,
            413 => GoogleV2Error::MaxTextSizeExceeded,
            422 => GoogleV2Error::CouldNotTranslate,
            501 => GoogleV2Error::TranslationDirectionNotSupported,
            other => GoogleV2Error::UnknownErrorCode(other),
        }
    }

    fn to_error_code(&self) -> u16 {
        match self {
            GoogleV2Error::InvalidAPIKey => 401,
            GoogleV2Error::BlockedAPIKey => 402,
            GoogleV2Error::DailyLimitExceeded => 404,
            GoogleV2Error::MaxTextSizeExceeded => 413,
            GoogleV2Error::CouldNotTranslate => 422,
            GoogleV2Error::TranslationDirectionNotSupported => 501,
            GoogleV2Error::UnknownErrorCode(other) => *other,
        }
    }
}

impl std::fmt::Display for GoogleV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error : {}", &self)
    }
}

impl std::error::Error for GoogleV2Error {}
