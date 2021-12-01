use uuid::Uuid;
use crate::common::Color;

trait PinCategory {
    fn get_color() -> Color;
}

#[derive(Debug)]
struct Link(String, Uuid);

#[derive(Debug)]
pub struct Pin {
    name: String,
    pin_id: Uuid,
    pin_name: String,
    direction: PinDirection,
    linked_to: Vec<Link>,
}

#[derive(Debug)]
enum PinDirection {
    Input,
    Output,
}

pub enum Pins {
    Exec,
    Boolean,
    Byte,
    Class,
    SoftClass,
    Int,
    Int64,
    Float,
    Double,
    Name,
    Delegate,
    MCDelegate,
    Object,
    Interface,
    SoftObject,
    String,
    Text,
    Struct,
    Wildcard,
    Enum,
    FieldPath,
}
