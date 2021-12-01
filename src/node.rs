use crate::parser::{meta, object};
use crate::pin::{Pin, Pins};
use anyhow::{anyhow, Context, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct FunctionReference {
    member_name: String,
    self_context: bool,
}

#[derive(Debug)]
pub enum Nodes {
    Comment {
        width: i32,
        height: i32,
        comment: String,
    },
    CallFunction {
        function_reference: FunctionReference,
    },
    VariableGet {},
    CustomEvent {
        custom_function_name: String,
    },
    InputAxisEvent {
        custom_function_name: String,
    },
}

impl Nodes {
    pub fn parse_class_props(lines: &Vec<&str>) -> Result<Nodes> {
        let parts = lines[0].split(" ").collect::<Vec<&str>>();
        let class_parts: Vec<_> = parts
            .into_iter()
            .filter(|part| part.starts_with("Class="))
            .collect();

        if class_parts.len() != 1 {
            return Err(anyhow!("ノードタイプが決定できませんでした"));
        }

        let classname = object::parse_kv(class_parts[0], false)
            .context("ノードクラスのパースに失敗しました")?;

        let node_props = match classname.1 {
            "/Script/UnrealEd.EdGraphNode_Comment" => Some(Self::parse_comment_node(lines)),
            "/Script/BlueprintGraph.K2Node_CallFunction" => {
                Some(Self::parse_callfunction_node(lines))
            }
            "/Script/BlueprintGraph.K2Node_VariableGet" => {
                Some(Self::parse_variableget_node(lines))
            }
            "/Script/BlueprintGraph.K2Node_CustomEvent" => {
                Some(Self::parse_customevent_node(lines))
            }
            "/Script/BlueprintGraph.K2Node_InputAxisEvent" => {
                Some(Self::parse_inputaxisevent_node(lines))
            }
            _ => None,
        }
        .context("一致するノードクラスがありません")?;

        Ok(node_props.context(format!(
            "クラスプロパティのパースに失敗しました: '{}'",
            classname.1
        ))?)
    }

    fn parse_comment_node(lines: &Vec<&str>) -> Result<Nodes> {
        Ok(Nodes::Comment {
            width: object::choose_and_parse_kv(lines, "NodeWidth", false)?
                .1
                .parse()?,
            height: object::choose_and_parse_kv(lines, "NodeHeight", false)?
                .1
                .parse()?,
            comment: object::choose_and_parse_kv(lines, "NodeComment", true)?
                .1
                .to_owned(),
        })
    }

    fn parse_callfunction_node(lines: &Vec<&str>) -> Result<Nodes> {
        Ok(Nodes::CallFunction {
            function_reference: FunctionReference {
                member_name: "test".to_owned(),
                self_context: false,
            },
        })
    }

    fn parse_variableget_node(lines: &Vec<&str>) -> Result<Nodes> {
        Ok(Nodes::VariableGet {})
    }

    fn parse_customevent_node(lines: &Vec<&str>) -> Result<Nodes> {
        Ok(Nodes::CustomEvent {
            custom_function_name: object::choose_and_parse_kv(lines, "CustomFunctionName", true)?
                .1
                .to_owned(),
        })
    }

    fn parse_inputaxisevent_node(lines: &Vec<&str>) -> Result<Nodes> {
        Ok(Nodes::InputAxisEvent {
            custom_function_name: object::choose_and_parse_kv(lines, "CustomFunctionName", true)?
                .1
                .to_owned(),
        })
    }

    // fn parse_props(&self) -> Result<Nodes> {

    // }
}

#[derive(Debug)]
pub struct Node {
    name: String,
    pos_x: i32,
    pos_y: i32,
    guid: Uuid,
    pins: Vec<Pin>,
    pub props: Nodes,
}

impl Node {
    fn parse_common_props<'a>(lines: &'a Vec<&str>) -> Result<(&'a str, i32, i32, Uuid)> {
        let candidates: Vec<&str> = lines[0]
            .split(" ")
            .map(|part| part.trim())
            .filter(|part| part.starts_with("Name="))
            .collect();
        if candidates.len() != 1 {
            return Err(anyhow!("ノード名が決定できませんでした"));
        }
        let nodename = object::parse_kv(candidates.first().unwrap(), true)
            .context("ノード名のパースに失敗しました")?
            .1;

        let raw_node_pos_x = object::choose_and_parse_kv(lines, "NodePosX", false)?
            .1
            .trim();
        let node_pos_x = raw_node_pos_x
            .parse()
            .context(format!("値のパースに失敗しました: '{}'", raw_node_pos_x))?;

        let raw_node_pos_y = object::choose_and_parse_kv(lines, "NodePosY", false)?
            .1
            .trim();
        let node_pos_y = raw_node_pos_y
            .parse()
            .context(format!("値のパースに失敗しました: '{}'", raw_node_pos_y))?;
        let guid = object::choose_and_parse_kv(lines, "NodeGuid", false)?
            .1
            .trim();

        Ok((
            nodename,
            node_pos_x,
            node_pos_y,
            Uuid::parse_str(guid).context("NodeGuidのパースに失敗しました")?,
        ))
    }

    pub fn parse_from_object_code(object_code: &str) -> Result<Self> {
        let raw_lines = object_code.split("\n").collect::<Vec<&str>>();
        let lines: Vec<_> = raw_lines.into_iter().filter(|line| line != &"").collect();

        if lines.len() < 3 {
            return Err(anyhow!("オブジェクトは最低限の長さを持っていません"));
        }
        if !lines.first().unwrap().starts_with("Begin Object") {
            return Err(anyhow!("オブジェクトが Begin Object から開始していません"));
        }
        if lines.last().unwrap().trim() != "End Object" {
            return Err(anyhow!("オブジェクトが End Object で終了していません"));
        }

        let common_props = Self::parse_common_props(&lines)
            .context("ノード共通プロパティのパースに失敗しました")?;

        Ok(Node {
            name: common_props.0.to_string(),
            pos_x: common_props.1,
            pos_y: common_props.2,
            guid: common_props.3,
            pins: Vec::new(),
            props: Nodes::parse_class_props(&lines)
                .context("ノードクラスプロパティのパースに失敗しました")?,
        })
    }
}
