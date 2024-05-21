use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use magnus::{function, prelude::*, Error, Ruby};

fn detect(subject: String) -> String {
    let detector: LanguageDetector = LanguageDetectorBuilder::from_all_languages().build();
    let detected_language: Option<Language> = detector.detect_language_of(subject);

    match detected_language {
        Some(language) => language.to_string(),
        None => "".to_string(),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Lingua")?;
    module.define_singleton_method("detect", function!(detect, 1))?;
    Ok(())
}
