#[derive(Debug)]
pub enum InputLanguage {
    Automatic,
    Defined(Language),
}

#[derive(Debug)]
pub enum Language {
    English,
    French,
    Spanish,
    Italian,
    Japanese,
    Esperanto,
    Nederlands,
    Portugues,
}

impl Language {
    pub fn to_language_code(&self) -> &'static str {
        use Language::*;
        match *self {
            English => "en",
            French => "fr",
            Spanish => "es",
            Italian => "it",
            Japanese => "ja",
            Esperanto => "eo",
            Nederlands => "nl",
            Portugues => "pt",
        }
    }
}
