use wasm_bindgen::prelude::*;
use tinymt::tinymt32;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};

/**
 * JavaScriptのDate.now関数をRustから呼び出す
 */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;
}
/**
 * 数値加算関数
 */
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
/**
 * 現在のタイムスタンプを取得
 */
#[wasm_bindgen]
pub fn get_timestamp() -> f64 {
    unsafe {
        now()
    }
}
/**
 * 乱数生成関数
 */
#[wasm_bindgen]
pub fn rand() -> u32 {
    let param = tinymt32::Param {
        mat1: 0x8F7011EE,
        mat2: 0xFC78FF1F,
        tmat: 0x3793fdff,
    };
    let seed = 1;
    tinymt32::from_seed(param, seed).gen()
}
/**
 * 文字列長さ
 */
#[wasm_bindgen]
pub fn get_string_length(input: &str) -> usize {
    input.len()
}
/**
 * Responseを整形
 */
#[wasm_bindgen]
pub fn modify_response(title: &str, completed: bool) -> JsValue {
    let modify_title = format!("Modified: {}", title);
    let modify_status = !completed; // 完了状態反転

    JsValue::from_serde(&serde_json::json!({
        "title": modify_title,
        "completed": modify_status,
    })).unwrap()
}
/**
 * 入力文字列カウント
 */
#[wasm_bindgen]
pub fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}
/**
 * 検索の機構を作る
 */
#[derive(Serialize, Deserialize)]
pub struct Person {
    id: u32,
    name: String,
    sex: u8, // 1: 男性, 2: 女性, 3: 不明
    age: u8,
    hobbies: Vec<String>,
}

// 趣味の定数リスト
const HOBBIES_DEVELOPMENT: &str = "開発";
const HOBBIES_MUSIC: &str = "音楽";
const HOBBIES_SPORTS: &str = "スポーツ";
const HOBBIES_TRAVEL: &str = "旅行";
const HOBBIES_READING: &str = "読書";
const HOBBIES_GAMING: &str = "ゲーム";

#[wasm_bindgen]
pub fn search_by_criteria(
    name_query: &str, 
    sex_query: u8,
    age_query: Option<u8>,
    hobby_query: Option<String>
) -> JsValue {
    let people = vec![
        Person { id: 1, name: "kazuya".to_string(), sex: 1, age: 28, hobbies: vec![HOBBIES_DEVELOPMENT.to_string(), HOBBIES_MUSIC.to_string()] },
        Person { id: 2, name: "tarou".to_string(), sex: 1, age: 30, hobbies: vec![HOBBIES_SPORTS.to_string(), HOBBIES_GAMING.to_string()] },
        Person { id: 3, name: "riko".to_string(), sex: 2, age: 30, hobbies: vec![HOBBIES_READING.to_string(), HOBBIES_MUSIC.to_string()] },
        Person { id: 4, name: "kinako".to_string(), sex: 3, age: 2, hobbies: vec![HOBBIES_TRAVEL.to_string()] },
        Person { id: 5, name: "kazuma".to_string(), sex: 1, age: 28, hobbies: vec![HOBBIES_SPORTS.to_string(), HOBBIES_DEVELOPMENT.to_string()] },
    ];

    let result: Vec<&Person> = people.iter()
        .filter(|&person| 
            person.name.to_lowercase().contains(&name_query.to_lowercase()) &&
            person.sex == sex_query &&
            age_query.map_or(true, |age| person.age == age) &&
            hobby_query.as_ref().map_or(true, |hobby| person.hobbies.iter().any(|h| h.to_lowercase() == hobby.to_lowercase()))
        )
        .collect();

    JsValue::from_serde(&result).unwrap()
}
