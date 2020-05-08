pub mod api;
pub mod translate_shell;

mod engines;
mod languages;

pub use engines::*;
pub use languages::*;

/// Enum containing different errors that may be raised by the program at runtime.
#[derive(Debug)]
pub enum Error {
    /// Error raised by std::process::Command itself when launching `trans`.
    CouldNotLaunchTranslateShell(String),
    /// Error raised by `trans` returning a non-zero exit code, contains `stderr`.
    TranslateShellProcessError(Vec<u8>),
    /// Errors when trying to convert translation result to utf-8.
    CouldNotConvertToUtf8String(std::string::FromUtf8Error),
    CouldNotConvertToUtf8Str(std::str::Utf8Error),
    /// Error when deserializing JSON string
    CouldNotDerializeJson,
    /// Error when sending API request : no KEY set
    NoApiKeySet,
    /// Error parsing query to a valid URI
    CouldNotParseUri(String),
    /// Error executing `tokio::runtime::Runtime::new()`
    FailedToCreateTokioRuntime,
    /// Yandex API error
    YandexAPIError(api::YandexError),
}
