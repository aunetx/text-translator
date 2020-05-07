use crate::*;
use std::process::Command;

#[derive(Debug)]
pub enum Translator {
    Google,
    Yandex,
    Bing,
    Spell,
    Aspell,
    Hunspell,
    Apertium,
}

impl Translator {
    fn get_name(&self) -> &'static str {
        use Translator::*;
        match *self {
            Google => "google",
            Yandex => "yandex",
            Bing => "bing",
            Spell => "spell",
            Aspell => "aspell",
            Hunspell => "hunspell",
            Apertium => "apertium",
        }
    }
}

impl Translator {
    pub fn translate(
        &self,
        text: String,
        source_language: LanguageType,
        target_language: Language,
    ) -> Result<String, Error> {
        let mut command = Command::new("trans");

        command.args(&[
            // misc
            "-brief",
            // set engine
            "-e",
            self.get_name(),
            // set output language
            "-t",
            target_language.to_language_code(),
        ]);

        match source_language {
            LanguageType::Automatic => (),
            LanguageType::Defined(language) => {
                command.args(&["-s", language.to_language_code()]);
            }
        };

        command.arg(text);

        let result = match command.output() {
            Ok(res) => res,
            Err(err) => return Err(Error::CouldNotLaunchTranslateShell(err.to_string())),
        };

        // check result
        match result.status.success() {
            false => Err(Error::TranslateShellProcessError(result.stderr)),
            true => match String::from_utf8(result.stdout) {
                Ok(res) => Ok(res),
                Err(err) => Err(Error::CouldNotConvertToUtf8(err)),
            },
        }
    }
}
