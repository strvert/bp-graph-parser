// use crate::pin::Pin;
// use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct FunctionReference {
    pub member_name: String,
    pub self_context: bool,
}

#[derive(Debug, PartialEq)]
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
        input_axis_name: String,
        override_parent_binding: bool,
    },
}

// #[derive(Debug)]
// pub struct Node {
//     pub class: String,
//     pub name: String,
//     pub pos_x: i32,
//     pub pos_y: i32,
//     pub guid: Uuid,
//     pub pins: Vec<Pin>,
//     pub props: Nodes,
// }
