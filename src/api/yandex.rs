use http::uri::Uri;
use hyper::{body::to_bytes, client::Client};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::runtime::Runtime;
use urlencoding::encode;

use super::*;

pub struct Yandex<'a> {
    key: Option<&'a str>,
}

impl<'a> Yandex<'a> {
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

const BASE_URL: &'static str = "https://translate.yandex.net/api/v1.5/tr.json/";

impl<'a> Api for Yandex<'a> {
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

#[derive(Debug)]
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
}
