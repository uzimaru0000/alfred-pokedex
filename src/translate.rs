use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Translate {
    id: i64,
    jp: String,
    en: String,
    ge: String,
    fr: String,
    ko: String,
    zh_cn: String,
    zh_tw: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TranslateCode {
    Jp,
    En,
    Ge,
    Fr,
    Ko,
    ZhCn,
    ZhTw,
}

impl Translate {
    fn get(&self, code: TranslateCode) -> &String {
        match code {
            TranslateCode::Jp => &self.jp,
            TranslateCode::En => &self.en,
            TranslateCode::Ge => &self.ge,
            TranslateCode::Fr => &self.fr,
            TranslateCode::Ko => &self.ko,
            TranslateCode::ZhCn => &self.zh_cn,
            TranslateCode::ZhTw => &self.zh_tw,
        }
    }
}

fn get_translate_map() -> HashMap<i64, Translate> {
    let json = include_str!("../poke.json");
    let data: Vec<Translate> = serde_json::from_str(json).unwrap();
    data.iter().map(|x| (x.id, x.clone())).collect()
}

pub fn translate_name(id: i64, code: TranslateCode) -> Option<String> {
    let map = get_translate_map();
    let trans = map.get(&id)?;

    Some(trans.get(code).clone())
}

pub fn search(query: String, code: TranslateCode) -> Vec<i64> {
    let map = get_translate_map();
    map.iter()
        .map(|(id, x)| (id, x.get(code).clone()))
        .filter(|(_, x)| x.contains(&query))
        .map(|(&id, _)| id)
        .collect::<Vec<i64>>()
}
