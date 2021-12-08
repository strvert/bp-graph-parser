use anyhow::{Context, Result};
use node::Node;

pub mod common;
pub mod node;
pub mod parser;
pub mod pin;

pub fn parse_graph(_graph_code: &str) -> String {
    "".to_string()
    // let tokens = parser::tokenize(&graph_code);
    // "hoge".to_string()
}

// pub fn to_json(k2node_code: &str) -> Result<String> {
//     let node =
//         Node::from_object_code(k2node_code).context("オブジェクトの構築に失敗しました")?;
//     println!("{:#?}", node);
//     Ok("aa".to_owned())
// }

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::fs;

    // #[test]
    // fn parse() {
    //     let _result = fs::read_dir("./k2node_codes").unwrap().map(|entry| {
    //         let path = entry.unwrap().path();
    //         let code = fs::read_to_string(path.to_str().unwrap()).unwrap();
    //         let json = to_json(&code);
    //         assert!(json.is_err(), "ファイルのパースに失敗しました");
    //         json
    //     });
    // }
}
