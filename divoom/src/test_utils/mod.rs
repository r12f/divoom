use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::{env, fs};
use crate::DivoomImageAnimation;

pub fn assert_object_equal_with_baseline<T: Serialize + DeserializeOwned + PartialEq + Debug>(
    actual: &T,
    reference_file_path: &str,
) {
    if env::var("DIVOOM_API_GENERATE_TEST_DATA").is_ok() {
        let actual_in_json_text =
            serde_json::to_string_pretty(&actual).expect("Serialize actual data into json failed!");

        fs::write(reference_file_path, actual_in_json_text).unwrap_or_else(|_| {
            panic!(
                "Generate test data file failed! Path = {}",
                reference_file_path
            )
        });

        return;
    }

    let expected_in_json_text =
        fs::read_to_string(reference_file_path).expect("Reading reference file failed!");

    let expected: T =
        serde_json::from_str(&expected_in_json_text).expect("Parsing reference data failed!");

    assert_eq!(actual, &expected);
}

pub fn assert_animation_equal_with_baseline(animation: &DivoomImageAnimation, reference_file_path: &str) {
    let mut actual_gif: Vec<u8> = Vec::new();
    animation.save_gif(&mut actual_gif).unwrap();

    if env::var("DIVOOM_API_GENERATE_TEST_DATA").is_ok() {
        fs::write(reference_file_path, actual_gif).unwrap_or_else(|_| {
            panic!(
                "Generate test data file failed! Path = {}",
                reference_file_path
            )
        });

        return;
    }

    let expected_gif = fs::read(reference_file_path).unwrap();
    assert_eq!(actual_gif, expected_gif);
}