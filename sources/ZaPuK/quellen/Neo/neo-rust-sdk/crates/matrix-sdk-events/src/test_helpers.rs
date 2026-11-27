// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::{fmt::Display, fs::File, io::BufReader};

use jsonschema::{Uri, ValidationError, Validator};

use serde::{Deserialize, Serialize};
use serde_json::Value;

lazy_static::lazy_static! {
    static ref VALIDATOR: Validator = make_validator();
}

#[derive(Debug)]
pub struct InvalidError {
    message: String,
}

impl Display for InvalidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid: {}", self.message)
    }
}

impl From<ValidationError<'_>> for InvalidError {
    fn from(error: ValidationError<'_>) -> Self {
        InvalidError {
            message: error.to_string(),
        }
    }
}

impl std::error::Error for InvalidError {}

#[derive(Debug)]
struct LoadingFileFailed {
    path: String,
    error: Box<dyn std::error::Error + Send + Sync>,
}

impl Display for LoadingFileFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to load file: {} with error: {}",
            self.path, self.error
        )
    }
}

impl std::error::Error for LoadingFileFailed {}

struct LocalYamlRetriever;

/// Load a YAML file from the local filesystem.
impl jsonschema::Retrieve for LocalYamlRetriever {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let path = uri.path();
        let file = File::open(format!("schema{path}")).map_err(|e| LoadingFileFailed {
            path: path.to_string(),
            error: Box::new(e),
        })?;
        let reader = BufReader::new(file);
        Ok(serde_saphyr::from_reader(reader)?)
    }
}

pub fn get_validator() -> &'static Validator {
    &VALIDATOR
}

static ALL_VALIDATOR_STR: &str = "
$schema: https://json-schema.org/draft/2020-12/schema


title: All supported events validators
description: References all supported events as a single validator

anyOf:
  - $ref: m.text.yaml
  - $ref: dev.fitko.neo.audio.v1.yaml
  - $ref: dev.fitko.neo.button.v1.yaml
  - $ref: dev.fitko.neo.file.v1.yaml
  - $ref: dev.fitko.neo.hyperlink.v1.yaml
  - $ref: dev.fitko.neo.image.v1.yaml
  - $ref: dev.fitko.neo.infobox.v1.yaml
  - $ref: dev.fitko.neo.file.v1.yaml
  - $ref: dev.fitko.neo.hyperlink.v1.yaml
  - $ref: dev.fitko.neo.image.v1.yaml
  - $ref: dev.fitko.neo.inquiry.dev.fitko.neo.file.v1.v1.yaml
  - $ref: dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1.yaml
  - $ref: dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1.yaml
  - $ref: dev.fitko.neo.label.v1.yaml
  - $ref: dev.fitko.neo.multi.length.v1.yaml
  - $ref: dev.fitko.neo.poll.response.v1.yaml
  - $ref: dev.fitko.neo.poll.v1.yaml
  - $ref: dev.fitko.neo.thumbnail.v1.yaml
  - $ref: dev.fitko.neo.title.v1.yaml
  - $ref: dev.fitko.neo.video.v1.yaml

  # metadata events
  - $ref: dev.fitko.neo.state.meta.case.v1.yaml

  # state events
  - $ref: dev.fitko.neo.meta.case_id.v1.yaml
  ";

/// Load the validator from the local filesystem, resolving relative references.
fn make_validator_inner() -> Result<Validator, Box<dyn std::error::Error>> {
    let v: Value = serde_saphyr::from_str(ALL_VALIDATOR_STR)?;
    let retriever = LocalYamlRetriever;
    Ok(jsonschema::options()
        .offline()
        .should_validate_formats(true)
        .with_retriever(retriever)
        .build(&v)?)
}

fn make_validator() -> Validator {
    make_validator_inner().unwrap()
}

/// Load an example json file into JSON Value
pub fn get_example(file_name: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let path = format!("schema/examples/{file_name}.json");
    let file = File::open(&path).map_err(|e| LoadingFileFailed {
        path,
        error: Box::new(e),
    })?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

/// Assert that a value is valid according to the schema
pub fn assert_valid<A: Serialize>(value: A) -> Result<(), InvalidError> {
    let serialized = serde_json::to_value(&value).map_err(|e| InvalidError {
        message: e.to_string(),
    })?;

    VALIDATOR.validate(&serialized).map_err(|e| InvalidError {
        message: e.to_string(),
    })
}

/// Assert that serializing and deserializing a value roundtrips correctly
pub fn assert_serialization_roundtrip<
    A: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
>(
    value: A,
) -> Result<(), InvalidError> {
    let serialized = serde_json::to_value(&value).map_err(|e| InvalidError {
        message: e.to_string(),
    })?;
    let deserialized: A = serde_json::from_value(serialized).map_err(|e| InvalidError {
        message: e.to_string(),
    })?;
    assert_eq!(value, deserialized);
    Ok(())
}
