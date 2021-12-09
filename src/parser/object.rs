use crate::parser::prop::prop_custom_props;

use super::{ast::Object, prop::prop_kv};
use nom::{branch::permutation, character::complete::multispace0, combinator::opt, Err, IResult};

pub fn object_content(s: &str) -> IResult<&str, Object> {
    let mut remain = s;
    let mut retobj = Object {
        props: Vec::new(),
        custom_props: Vec::new(),
    };

    loop {
        let r = opt(permutation((multispace0, prop_kv, multispace0)))(remain).unwrap();
        if let Some(p) = r.1 {
            remain = r.0;
            retobj.props.push(p.1);
            continue;
        }

        let r = opt(permutation((multispace0, prop_custom_props, multispace0)))(remain).unwrap();
        if let Some(p) = r.1 {
            remain = r.0;
            retobj.custom_props.push(p.1);
            continue;
        }
        break;
    }
    Ok((remain, retobj))
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::{CustomProp, CustomPropValue, LinkedTo, Prop, PropValue};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn parse_object_content() {
        let sample = r#"CustomFunctionName="MyEvent"
   NodePosX=512
   NodePosY=-16
   NodeGuid=741BFB8C4AF2854BAE60B3B660D5B625
   CustomProperties Pin (PinId=7CD635904148E54F000DA597BA60AB39,Direction="EGPD_Output",PinType.PinSubCategoryObject=None,PinType.PinValueType=(),PinType.bIsReference=False,)
   CustomProperties Pin (PinType.PinSubCategoryObject=Class'"/Script/UMG.Button"',PinType.PinSubCategoryMemberReference=(),PinType.ContainerType=None,PinType.bIsReference=True,LinkedTo=(K2Node_VariableGet_17 570BAD4542CBB0285413EEAB4F6DBDDA,))"#;
        assert_eq!(
            object_content(sample),
            Ok((
                "",
                Object {
                    props: vec![
                        Prop {
                            key: "CustomFunctionName".to_owned(),
                            value: PropValue::String("MyEvent".to_owned())
                        },
                        Prop {
                            key: "NodePosX".to_owned(),
                            value: PropValue::Integer(512)
                        },
                        Prop {
                            key: "NodePosY".to_owned(),
                            value: PropValue::Integer(-16)
                        },
                        Prop {
                            key: "NodeGuid".to_owned(),
                            value: PropValue::Uuid(
                                Uuid::parse_str("741BFB8C4AF2854BAE60B3B660D5B625").unwrap()
                            )
                        },
                    ],
                    custom_props: vec![
                        CustomProp {
                            domain: "Pin".to_owned(),
                            value: CustomPropValue::Pin(vec![
                                Prop {
                                    key: "PinId".to_owned(),
                                    value: PropValue::Uuid(
                                        Uuid::parse_str("7CD635904148E54F000DA597BA60AB39")
                                            .unwrap()
                                    ),
                                },
                                Prop {
                                    key: "Direction".to_owned(),
                                    value: PropValue::String("EGPD_Output".to_owned())
                                },
                                Prop {
                                    key: "PinType.PinSubCategoryObject".to_owned(),
                                    value: PropValue::Object("None".to_owned(), "None".to_owned())
                                },
                                Prop {
                                    key: "PinType.PinValueType".to_owned(),
                                    value: PropValue::PropList(Vec::new())
                                },
                                Prop {
                                    key: "PinType.bIsReference".to_owned(),
                                    value: PropValue::Boolean(false)
                                },
                            ])
                        },
                        CustomProp {
                            domain: "Pin".to_owned(),
                            value: CustomPropValue::Pin(vec![
                                Prop {
                                    key: "PinType.PinSubCategoryObject".to_owned(),
                                    value: PropValue::Object(
                                        "Class".to_owned(),
                                        "/Script/UMG.Button".to_owned()
                                    )
                                },
                                Prop {
                                    key: "PinType.PinSubCategoryMemberReference".to_owned(),
                                    value: PropValue::PropList(Vec::new())
                                },
                                Prop {
                                    key: "PinType.ContainerType".to_owned(),
                                    value: PropValue::Object("None".to_owned(), "None".to_owned())
                                },
                                Prop {
                                    key: "PinType.bIsReference".to_owned(),
                                    value: PropValue::Boolean(true)
                                },
                                Prop {
                                    key: "LinkedTo".to_owned(),
                                    value: PropValue::LinkedToList(vec![LinkedTo {
                                        name: "K2Node_VariableGet_17".to_owned(),
                                        uuid: Uuid::parse_str("570BAD4542CBB0285413EEAB4F6DBDDA")
                                            .unwrap()
                                    }])
                                },
                            ])
                        },
                    ]
                }
            ))
        );
    }
}
