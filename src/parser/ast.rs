use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum PropValue {
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
    Uuid(Uuid),
    NslocText(String, String, String),
    Object(String, String),
    PropList(Vec<Prop>),
}

#[derive(Debug, PartialEq)]
pub struct Prop {
    pub key: String,
    pub value: PropValue,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub props: Vec<Prop>,
}
