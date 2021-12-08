use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct LinkedTo {
    pub name: String,
    pub uuid: Uuid,
}

#[derive(Debug, PartialEq)]
pub enum PropValue {
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
    Uuid(Uuid),
    NslocText(String, String, String),
    Object(String, String),
    LinkedToList(Vec<LinkedTo>),
    PropList(Vec<Prop>),
}

#[derive(Debug, PartialEq)]
pub struct Prop {
    pub key: String,
    pub value: PropValue,
}

#[derive(Debug, PartialEq)]
pub enum CustomPropValue {
    Pin(Vec<Prop>),
}

#[derive(Debug, PartialEq)]
pub struct CustomProp {
    pub name: String,
    pub value: CustomPropValue,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub props: Vec<Prop>,
    pub custom_props: Vec<CustomProp>,
}
