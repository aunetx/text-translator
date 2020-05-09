use rayon::iter::ParallelBridge;
use rayon::prelude::*;

use text_translator::*;

const TEXT: &str = "Hello, my name is Naruto Uzumaki!\nI love noodles and fights.";

use translate_shell::Translator::*;
const ENGINES: [translate_shell::Translator; 7] =
    [Google, Yandex, Bing, Spell, Aspell, Hunspell, Apertium];

#[test]
fn translate_shell_single_translate() {
    static mut FAILED_TASKS: u32 = 0;

    for engine in ENGINES.iter() {
        let res = engine.translate(
            TEXT.to_string(),
            InputLanguage::Defined(Language::English),
            Language::French,
        );

        match res {
            Ok(translation) => {
                println!("[{:?}] Translated to English : {:#?}", engine, translation)
            }
            Err(err) => {
                println!("[{:?}] Could not translate to English : {:?}", engine, err);
                unsafe { FAILED_TASKS += 1 }
            }
        }
    }

    //assert_eq!(0, unsafe { FAILED_TASKS })
}

#[test]
fn translate_shell_single_translate_automatic_language() {
    static mut FAILED_TASKS: u32 = 0;

    for engine in ENGINES.iter() {
        let res = engine.translate(TEXT.to_string(), InputLanguage::Automatic, Language::French);

        match res {
            Ok(translation) => {
                println!("[{:?}] Translated to English : {:#?}", engine, translation)
            }
            Err(err) => {
                println!("[{:?}] Could not translate to English : {:?}", engine, err);
                unsafe { FAILED_TASKS += 1 }
            }
        }
    }

    //assert_eq!(0, unsafe { FAILED_TASKS })
}

#[test]
fn translate_shell_translate_all_languages() {
    static mut FAILED_TASKS: u32 = 0;

    const ENGINE: Engine = Engine::TranslateShell(translate_shell::Translator::Yandex);

    Language::iterator().par_bridge().for_each(|language| {
        let res = ENGINE.translate(TEXT.to_string(), InputLanguage::Automatic, language.clone());

        match res {
            Ok(translation) => println!("Translated to {:?} : {:#?}", language, translation),
            Err(err) => {
                println!("Could not translate to {:?} : {:#?}", language, err);
                unsafe { FAILED_TASKS += 1 }
            }
        }
    });

    //assert_eq!(0, unsafe { FAILED_TASKS })
}
