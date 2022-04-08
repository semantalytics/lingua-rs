/*
 * Copyright © 2020-today Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(non_snake_case)]

use crate::builder::{MINIMUM_RELATIVE_DISTANCE_MESSAGE, MISSING_LANGUAGE_MESSAGE};
use crate::{IsoCode639_1, IsoCode639_3, Language, LanguageDetector as Detector};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashSet;
use std::str::FromStr;
#[cfg(feature = "wasm")] 
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct LanguageDetectorBuilder {
    languages: HashSet<Language>,
    minimum_relative_distance: f64,
    is_every_language_model_preloaded: bool,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct LanguageDetector {
    detector: Detector,
}

#[derive(Serialize)]
pub struct ConfidenceValue {
    language: String,
    confidence: f64,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl LanguageDetectorBuilder {
    /// Creates and returns an instance of `LanguageDetectorBuilder` with all built-in languages.
    pub fn fromAllLanguages() -> Self {
        Self::from(Language::all())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in spoken languages.
    pub fn fromAllSpokenLanguages() -> Self {
        Self::from(Language::all_spoken_ones())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Arabic script.
    pub fn fromAllLanguagesWithArabicScript() -> Self {
        Self::from(Language::all_with_arabic_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Cyrillic script.
    pub fn fromAllLanguagesWithCyrillicScript() -> Self {
        Self::from(Language::all_with_cyrillic_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Devanagari script.
    pub fn fromAllLanguagesWithDevanagariScript() -> Self {
        Self::from(Language::all_with_devanagari_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages supporting the Latin script.
    pub fn fromAllLanguagesWithLatinScript() -> Self {
        Self::from(Language::all_with_latin_script())
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with all built-in languages except those specified in `languages`.
    ///
    /// ⚠ Throws an error if less than two `languages` are used to build
    /// the `LanguageDetector`.
    pub fn fromAllLanguagesWithout(
        languages: Box<[JsValue]>,
    ) -> Result<LanguageDetectorBuilder, JsValue> {
        let mut languages_to_load = Language::all();
        let languages_to_filter_out = languages
            .iter()
            .filter_map(|it| it.as_string())
            .filter_map(|it| Language::from_str(&it).ok())
            .collect_vec();
        languages_to_load.retain(|it| !languages_to_filter_out.contains(it));

        if languages_to_load.len() < 2 {
            return Err(JsValue::from(MISSING_LANGUAGE_MESSAGE));
        }

        Ok(Self::from(languages_to_load))
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with the specified `languages`.
    ///
    /// ⚠ Throws an error if less than two `languages` are specified.
    pub fn fromLanguages(languages: Box<[JsValue]>) -> Result<LanguageDetectorBuilder, JsValue> {
        let selected_languages = languages
            .iter()
            .filter_map(|it| it.as_string())
            .filter_map(|it| Language::from_str(&it).ok())
            .collect::<HashSet<Language>>();

        if selected_languages.len() < 2 {
            return Err(JsValue::from(MISSING_LANGUAGE_MESSAGE));
        }

        Ok(Self::from(selected_languages))
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with the languages specified by the respective ISO 639-1 codes.
    ///
    /// ⚠ Throws an error if less than two `iso_codes` are specified.
    pub fn fromISOCodes6391(isoCodes: Box<[JsValue]>) -> Result<LanguageDetectorBuilder, JsValue> {
        let selected_iso_codes = isoCodes
            .iter()
            .filter_map(|it| it.as_string())
            .filter_map(|it| IsoCode639_1::from_str(&it).ok())
            .collect_vec();

        if selected_iso_codes.len() < 2 {
            return Err(JsValue::from(MISSING_LANGUAGE_MESSAGE));
        }

        let selected_languages = selected_iso_codes
            .iter()
            .map(Language::from_iso_code_639_1)
            .collect::<HashSet<_>>();

        Ok(Self::from(selected_languages))
    }

    /// Creates and returns an instance of `LanguageDetectorBuilder`
    /// with the languages specified by the respective ISO 639-3 codes.
    ///
    /// ⚠ Throws an error if less than two `iso_codes` are specified.
    pub fn fromISOCodes6393(isoCodes: Box<[JsValue]>) -> Result<LanguageDetectorBuilder, JsValue> {
        let selected_iso_codes = isoCodes
            .iter()
            .filter_map(|it| it.as_string())
            .filter_map(|it| IsoCode639_3::from_str(&it).ok())
            .collect_vec();

        if selected_iso_codes.len() < 2 {
            return Err(JsValue::from(MISSING_LANGUAGE_MESSAGE));
        }

        let selected_languages = selected_iso_codes
            .iter()
            .map(Language::from_iso_code_639_3)
            .collect::<HashSet<_>>();

        Ok(Self::from(selected_languages))
    }

    /// Sets the desired value for the minimum relative distance measure.
    ///
    /// By default, *Lingua* returns the most likely language for a given
    /// input text. However, there are certain words that are spelled the
    /// same in more than one language. The word *prologue*, for instance,
    /// is both a valid English and French word. Lingua would output either
    /// English or French which might be wrong in the given context.
    /// For cases like that, it is possible to specify a minimum relative
    /// distance that the logarithmized and summed up probabilities for
    /// each possible language have to satisfy.
    ///
    /// Be aware that the distance between the language probabilities is
    /// dependent on the length of the input text. The longer the input
    /// text, the larger the distance between the languages. So if you
    /// want to classify very short text phrases, do not set the minimum
    /// relative distance too high. Otherwise you will get most results
    /// returned as `undefined` which is the return value for cases
    /// where language detection is not reliably possible.
    ///
    /// ⚠ Throws an error if `distance` is smaller than 0.0 or greater than 0.99.
    pub fn setMinimumRelativeDistance(&mut self, distance: f64) -> Result<(), JsValue> {
        if !(0.0..=0.99).contains(&distance) {
            return Err(JsValue::from(MINIMUM_RELATIVE_DISTANCE_MESSAGE));
        }
        self.minimum_relative_distance = distance;
        Ok(())
    }

    /// Configures `LanguageDetectorBuilder` to preload all language models when creating
    /// the instance of [LanguageDetector].
    ///
    /// By default, *Lingua* uses lazy-loading to load only those language models
    /// on demand which are considered relevant by the rule-based filter engine.
    /// For web services, for instance, it is rather beneficial to preload all language
    /// models into memory to avoid unexpected latency while waiting for the
    /// service response. This method allows to switch between these two loading modes.
    pub fn enablePreloadingLanguageModels(&mut self) {
        self.is_every_language_model_preloaded = true;
    }

    /// Creates and returns the configured instance of [LanguageDetector].
    pub fn build(&mut self) -> LanguageDetector {
        LanguageDetector {
            detector: Detector::from(
                self.languages.clone(),
                self.minimum_relative_distance,
                self.is_every_language_model_preloaded,
            ),
        }
    }

    fn from(languages: HashSet<Language>) -> Self {
        Self {
            languages,
            minimum_relative_distance: 0.0,
            is_every_language_model_preloaded: false,
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl LanguageDetector {
    /// Detects the language of given input text.
    /// If the language cannot be reliably detected, `undefined` is returned.
    pub fn detectLanguageOf(&self, text: &str) -> Option<String> {
        match self.detector.detect_language_of(text) {
            Some(language) => Some(language.to_string()),
            None => None,
        }
    }

    /// Computes confidence values for each language considered possible for the given input text.
    ///
    /// An object of all possible languages is returned, sorted by their confidence value in
    /// descending order. The values that this method computes are part of a **relative**
    /// confidence metric, not of an absolute one. Each value is a number between 0.0 and 1.0.
    /// The most likely language is always returned with value 1.0. All other languages get values
    /// assigned which are lower than 1.0, denoting how less likely those languages are in
    /// comparison to the most likely language.
    ///
    /// The object returned by this method does not necessarily contain all languages which the
    /// calling instance of `LanguageDetector` was built from. If the rule-based engine decides
    /// that a specific language is truly impossible, then it will not be part of the returned
    /// object. Likewise, if no ngram probabilities can be found within the detector's languages
    /// for the given input text, the returned object will be empty. The confidence value for
    /// each language not being part of the returned object is assumed to be 0.0.
    pub fn computeLanguageConfidenceValues(&self, text: &str) -> JsValue {
        let confidence_values = self
            .detector
            .compute_language_confidence_values(text)
            .iter()
            .map(|(language, confidence)| ConfidenceValue {
                language: language.to_string(),
                confidence: *confidence,
            })
            .collect_vec();

        JsValue::from_serde(&confidence_values).unwrap()
    }
}
