//! Search for a kanji by its strokes, from the list of Joyo Kanji.
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct KanjiItem {
    // #[wasm_bindgen(getter_with_clone)]
    // #[serde(default)]
    // pub id: u32,
    #[wasm_bindgen(getter_with_clone)]
    #[serde(rename = "Kanji")]
    pub kanji: String,
    #[wasm_bindgen(getter_with_clone)]
    #[serde(rename = "Strokes")]
    pub strokes: u32,
    #[wasm_bindgen(getter_with_clone)]
    #[serde(rename = "On within Joyo")]
    pub onyomi: String,
    #[wasm_bindgen(getter_with_clone)]
    #[serde(rename = "Kun within Joyo")]
    pub kunyomi: String,
}

#[wasm_bindgen]
pub fn search_by_strokes(stroke_count: u32) -> Result<JsValue, JsValue> {
    // Load the CSV data
    let data = include_str!("../joyo_kanji.csv");
    
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';') // Use semicolon as delimiter if necessary
        .flexible(true)
        .from_reader(data.as_bytes());

    let mut filtered_kanji_list: Vec<KanjiItem> = Vec::new();

    // Deserialize and filter kanji by stroke count
    for result in rdr.deserialize::<KanjiItem>() {
        match result {
            Ok(kanji_item) => {
                if kanji_item.strokes == stroke_count {
                    filtered_kanji_list.push(kanji_item);
                }
            }
            Err(e) => {
                // Log errors for debugging
                log(&format!("Error parsing row: {}", e));
            }
        }
    }
    
    // Serialize the filtered list to JsValue and return
    serde_wasm_bindgen::to_value(&filtered_kanji_list)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Test case for loading data
    #[test]
    #[wasm_bindgen_test]
    fn test_load_kanji_data() {
        let data = include_str!("../joyo_kanji.csv");
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_reader(data.as_bytes());

        let mut kanji_list: Vec<KanjiItem> = Vec::new();

        for result in rdr.deserialize::<KanjiItem>() {
            match result {
                Ok(kanji_item) => kanji_list.push(kanji_item),
                Err(e) => panic!("Error parsing row: {}", e),
            }
        }

        // Check if data was loaded correctly
        assert!(!kanji_list.is_empty(), "Kanji list should not be empty");
    }

    // Test for the search_by_strokes function with a valid stroke count
    #[test]
    #[wasm_bindgen_test]
    fn test_search_by_strokes_valid() {
        let result = search_by_strokes(3).expect("Function should return Ok");
        let kanji_list: Vec<KanjiItem> = from_value(result).expect("Deserialization should succeed");

        // Check the results
        assert!(!kanji_list.is_empty(), "Kanji list should not be empty for stroke count 3");
        for kanji in kanji_list {
            assert_eq!(kanji.strokes, 3, "Kanji strokes should match the filter");
        }
    }

    // Test for the search_by_strokes function with an invalid stroke count
    #[test]
    #[wasm_bindgen_test]
    fn test_search_by_strokes_invalid() {
        let result = search_by_strokes(100).expect("Function should return Ok");
        let kanji_list: Vec<KanjiItem> = from_value(result).expect("Deserialization should succeed");

        // Check that no results are returned for an invalid stroke count
        assert!(kanji_list.is_empty(), "Kanji list should be empty for stroke count 100");
    }

    // Test for handling invalid CSV data
    #[test]
    #[wasm_bindgen_test]
    #[should_panic(expected = "Error parsing row:")]
    fn test_invalid_csv_data() {
        let invalid_csv = "Kanji;Strokes;On within Joyo;Kun within Joyo\n壱;a;ichi;ichi";
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_reader(invalid_csv.as_bytes());

        for result in rdr.deserialize::<KanjiItem>() {
            if let Err(e) = result {
                panic!("Error parsing row: {}", e);
            }
        }
    }

    // Test for serialization errors
    #[test]
    #[wasm_bindgen_test]
    fn test_serialization_error() {
        let data = vec![KanjiItem {
            kanji: "壱".to_string(),
            strokes: 3,
            onyomi: String::new(),
            kunyomi: String::new(),
        }];

        // Introduce an error in serialization by adding invalid data
        let result = serde_wasm_bindgen::to_value(&data);

        assert!(result.is_ok(), "Serialization should succeed for valid data");
    }
}