use anyhow::{Context, Result};
use node::Node;

pub mod common;
pub mod node;
pub mod parser;
pub mod pin;

pub fn to_json(k2node_code: &str) -> Result<String> {
    let node =
        Node::parse_from_object_code(k2node_code).context("オブジェクトの構築に失敗しました")?;
    println!("{:?}", node);
    Ok("aa".to_owned())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
