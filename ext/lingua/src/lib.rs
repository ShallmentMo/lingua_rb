use std::str::FromStr;

use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use magnus::{function, prelude::*, Error, Ruby, Symbol};

fn detect(arguments: magnus::RArray) -> Option<String> {
    match arguments.len() {
        1 => {
            let subject = arguments.shift::<String>().unwrap();
            let detector: LanguageDetector = LanguageDetectorBuilder::from_all_languages().build();
            let detected_language: Option<Language> = detector.detect_language_of(subject);

            detected_language.map(|language| language.to_string())
        }
        2 => {
            let subject = arguments.shift::<String>().unwrap();
            let options = arguments.shift::<magnus::RHash>().unwrap();
            let mut builder = match options.fetch::<Symbol, Vec<String>>(Symbol::new("languages")) {
                Ok(languages) => {
                    let languages: Vec<Language> = languages
                        .into_iter()
                        .filter_map(|l| Language::from_str(&l).ok())
                        .collect();
                    Some(LanguageDetectorBuilder::from_languages(&languages))
                }
                Err(_) => None,
            };
            if builder.is_none() {
                builder = match options.fetch::<&str, Vec<String>>("languages") {
                    Ok(languages) => {
                        let languages: Vec<Language> = languages
                            .into_iter()
                            .filter_map(|l| Language::from_str(&l).ok())
                            .collect();
                        Some(LanguageDetectorBuilder::from_languages(&languages))
                    }
                    Err(_) => None,
                };
            }
            let mut builder = builder.unwrap_or_else(LanguageDetectorBuilder::from_all_languages);
            if let Ok(minimum_relative_distance) =
                options.fetch::<&str, f64>("minimum_relative_distance")
            {
                builder.with_minimum_relative_distance(minimum_relative_distance);
            };
            if let Ok(minimum_relative_distance) =
                options.fetch::<Symbol, f64>(Symbol::new("minimum_relative_distance"))
            {
                builder.with_minimum_relative_distance(minimum_relative_distance);
            };
            if options
                .fetch::<&str, bool>("is_every_language_model_preloaded")
                .unwrap_or(false)
                || options
                    .fetch::<Symbol, bool>(Symbol::new("is_every_language_model_preloaded"))
                    .unwrap_or(false)
            {
                builder.with_preloaded_language_models();
            };
            if options
                .fetch::<&str, bool>("is_low_accuracy_mode_enabled")
                .unwrap_or(false)
                || options
                    .fetch::<Symbol, bool>(Symbol::new("is_low_accuracy_mode_enabled"))
                    .unwrap_or(false)
            {
                builder.with_low_accuracy_mode();
            };
            let detector = builder.build();
            let detected_language: Option<Language> = detector.detect_language_of(subject);

            detected_language.map(|language| language.to_string())
        }
        _ => None,
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Lingua")?;
    module.define_singleton_method("detect", function!(detect, -2))?;
    Ok(())
}
