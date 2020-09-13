use crate::*;

pub mod yandex;
pub use yandex::Yandex;

pub mod google_v2;
pub use google_v2::GoogleV2;

/// A trait defining a translate API.
///
/// Implements `new()` to return a new API, and `translate()` to translate a text.
pub trait Api {
    /// Returns a new API struct, without initiating it.
    fn new() -> Self;

    /// Translates text between two languages.
    ///
    /// Takes in input the selected text and two enums:
    ///
    /// - `source_language`: [`InputLanguage`](../enum.InputLanguage.html), representing either automatic language detection or a defined language;
    /// - `target_language`: [`Language`](../enum.Language.html), representing a defined language to output to.
    ///
    /// Returns a `Result` containing either a `String` with the translated text, or an [`Error`](../enum.Error.html) that happened during the process.
    fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error>;
}

/// Extends [`Api`](trait.Api.html) to implement language detection.
pub trait ApiDetect: Api {
    /// Detect the language of the selected text.
    ///
    /// ## Inputs
    ///
    /// Takes in input only a `String` containing the text.
    ///
    /// ## Outputs
    ///
    /// The method returns a `Result<Option<Language>, Error>` type:
    ///
    /// - if the API was able to detect the language, it will result in an `Ok(Some(detected_language))`.
    /// - if it failed to detect, it will be an `Ok(None)`.
    /// - if an error preventing the API to do the detection, it will return an error: `Error(returned_error)`.
    fn detect(&self, text: String) -> Result<Option<Language>, Error>;
}

/// Extends [`Api`](trait.Api.html), where the API needs to have an API Key.
pub trait ApiKey<'a>: Api + Sized {
    fn set_set(&mut self, key: &'a str);

    fn get_key(&self) -> Option<&'a str>;
}

trait ApiTranslateResponse {
    fn get_text(&self) -> String;
}

trait ApiDetectResponse {
    fn get_lang(&self) -> Option<Language>;
}

/// Used on enums representing errors that a call to an API returned.
pub trait ApiError {
    /// Converts an error code to the enum variant.
    fn from_error_code(code: u16) -> Self;

    /// Converts an error variant to the matching error code.
    fn to_error_code(&self) -> u16;
}
