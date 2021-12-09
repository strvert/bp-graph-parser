pub mod common;
pub mod node;
pub mod parser;
pub mod pin;

use parser::ast::Object;

pub fn parse_graph(graph_code: &str) -> Vec<Object> {
    parser::object::objects(graph_code).unwrap().1
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
