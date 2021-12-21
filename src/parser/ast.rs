use serde::Serialize;
use uuid::Uuid;

/// A structure that holds node pins and other connection destination information.
#[derive(Debug, PartialEq, Serialize)]
pub struct LinkedTo {
    pub name: String,
    pub uuid: Uuid,
}

/// An enumerated type that holds basic properties.
#[derive(Debug, PartialEq, Serialize)]
pub enum PropValue {
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
    Uuid(Uuid),
    NslocText(String, String, String),
    ObjectReference(String, String),
    LinkedToList(Vec<LinkedTo>),
    PropList(Vec<Prop>),
    Other(String),
}

/// An enumerated type that indicates the internal elements of an object.
#[derive(Debug, PartialEq, Serialize)]
pub enum ObjectElement {
    Prop(Prop),
    CustomProp(CustomProp),
    Object(Object),
}


/// A structure that represents the basic Key / Value properties.
#[derive(Debug, PartialEq, Serialize)]
pub struct Prop {
    pub key: String,
    pub value: PropValue,
}

/// A structure that indicates custom properties.
#[derive(Debug, PartialEq, Serialize)]
pub enum CustomPropValue {
    Pin(Vec<Prop>),
}

/// A structure of custom properties held by an object.
#[derive(Debug, PartialEq, Serialize)]
pub struct CustomProp {
    pub domain: String,
    pub value: CustomPropValue,
}

/// A structure that represents an object header.
#[derive(Debug, PartialEq, Serialize)]
pub struct ObjectHeader {
    pub object_type: String,
    pub header_props: Vec<Prop>,
}

/// A strucutre that represents an object.
#[derive(Debug, PartialEq, Serialize)]
pub struct Object {
    pub header: ObjectHeader,
    pub elements: Vec<ObjectElement>,
}

/// A strucutre that represents an object end.
#[derive(Debug, PartialEq, Serialize)]
pub struct ObjectEnd {
    pub object_type: String,
}


/// A structure that represents the Vec of an object.
#[derive(Debug, Serialize)]
pub struct Objects(pub Vec<Object>);
