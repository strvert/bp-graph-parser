use crate::parser::prop::prop_custom_props;

use super::{
    ast::{Object, ObjectElement, ObjectEnd, ObjectHeader, Prop, PropValue},
    prop::prop_kv,
};
use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_until1},
    character::complete::{alphanumeric1, line_ending, multispace0, space0},
    combinator::{eof, map, opt, recognize},
    multi::separated_list0,
    IResult,
};

pub fn object_content(s: &str) -> IResult<&str, Vec<ObjectElement>> {
    let mut remain = s;
    let mut retobj = Vec::new();

    loop {
        let r = opt(permutation((
            multispace0,
            alt((
                map(prop_kv, |v| ObjectElement::Prop(v)),
                map(prop_custom_props, |v| ObjectElement::CustomProp(v)),
                // map(object, |v| {
                //     ObjectElement::Object(v)
                // }),
            )),
            multispace0,
        )))(remain)
        .unwrap();
        if let Some(p) = r.1 {
            remain = r.0;
            retobj.push(p.1);
            continue;
        }
        break;
    }
    Ok((remain, retobj))
}

pub fn object_begin(s: &str) -> IResult<&str, ObjectHeader> {
    let (remain, (_, object_type)) = permutation((tag("Begin "), alphanumeric1))(s)?;

    let (remain, (_, props, _)) = permutation((
        space0,
        separated_list0(
            tag(" "),
            alt((
                map(
                    permutation((tag("Class="), take_until1(" "))),
                    |v: (_, &str)| Prop {
                        key: "Class".to_owned(),
                        value: PropValue::String(v.1.to_owned()),
                    },
                ),
                prop_kv,
            )),
        ),
        line_ending,
    ))(remain)?;

    Ok((
        remain,
        ObjectHeader {
            object_type: object_type.to_owned(),
            header_props: props,
        },
    ))
}

pub fn object_end(s: &str) -> IResult<&str, ObjectEnd> {
    map(
        permutation((
            tag("End "),
            alphanumeric1,
            alt((recognize(line_ending), recognize(eof))),
        )),
        |v: (_, &str, _)| ObjectEnd {
            object_type: v.1.to_owned(),
        },
    )(s)
}

pub fn object(s: &str) -> IResult<&str, Object> {
    map(
        permutation((object_begin, object_content, object_end)),
        |v| Object {
            header: v.0,
            elements: v.1,
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{
        CustomProp, CustomPropValue, LinkedTo, ObjectElement, ObjectHeader, Prop, PropValue,
    };
    use uuid::Uuid;

    #[test]
    fn parse_nested_object() {
        let sample = r#"Begin Object
End Object"#;
    }

    #[test]
    fn parse_object() {
        let sample = r#"Begin Object Class=/Script/BlueprintGraph.K2Node_VariableGet Name="K2Node_VariableGet_1"
   VariableReference=(MemberName="InteractionDistance",MemberGuid=39364FF3470F9B07BCE5F6A5FB580445,bSelfContext=True)
   CustomFunctionName="MyEvent"
   NodePosX=512
   NodePosY=-16
   CustomProperties Pin (PinId=7CD635904148E54F000DA597BA60AB39,Direction="EGPD_Output",PinType.PinSubCategoryObject=None,PinType.PinValueType=(),PinType.bIsReference=False,)
   CustomProperties Pin (PinType.PinSubCategoryObject=Class'"/Script/UMG.Button"',PinType.PinSubCategoryMemberReference=(),PinType.ContainerType=None,PinType.bIsReference=True,LinkedTo=(K2Node_VariableGet_17 570BAD4542CBB0285413EEAB4F6DBDDA,))
End Object
"#;
        assert_eq!(
            object(sample),
            Ok((
                "",
                Object {
                    header: ObjectHeader {
                        object_type: "Object".to_owned(),
                        header_props: vec![
                            Prop {
                                key: "Class".to_owned(),
                                value: PropValue::String(
                                    "/Script/BlueprintGraph.K2Node_VariableGet".to_owned()
                                )
                            },
                            Prop {
                                key: "Name".to_owned(),
                                value: PropValue::String("K2Node_VariableGet_1".to_owned())
                            }
                        ]
                    },
                    elements: vec![
                        ObjectElement::Prop(Prop {
                            key: "VariableReference".to_string(),
                            value: PropValue::PropList(vec![
                                Prop {
                                    key: "MemberName".to_owned(),
                                    value: PropValue::String("InteractionDistance".to_owned())
                                },
                                Prop {
                                    key: "MemberGuid".to_owned(),
                                    value: PropValue::Uuid(
                                        Uuid::parse_str("39364FF3470F9B07BCE5F6A5FB580445")
                                            .unwrap()
                                    )
                                },
                                Prop {
                                    key: "bSelfContext".to_owned(),
                                    value: PropValue::Boolean(true)
                                }
                            ])
                        }),
                        ObjectElement::Prop(Prop {
                            key: "CustomFunctionName".to_owned(),
                            value: PropValue::String("MyEvent".to_owned())
                        }),
                        ObjectElement::Prop(Prop {
                            key: "NodePosX".to_string(),
                            value: PropValue::Integer(512)
                        }),
                        ObjectElement::Prop(Prop {
                            key: "NodePosY".to_string(),
                            value: PropValue::Integer(-16)
                        }),
                        ObjectElement::CustomProp(CustomProp {
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
                                    value: PropValue::ObjectReference(
                                        "None".to_owned(),
                                        "None".to_owned()
                                    )
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
                        }),
                        ObjectElement::CustomProp(CustomProp {
                            domain: "Pin".to_owned(),
                            value: CustomPropValue::Pin(vec![
                                Prop {
                                    key: "PinType.PinSubCategoryObject".to_owned(),
                                    value: PropValue::ObjectReference(
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
                                    value: PropValue::ObjectReference(
                                        "None".to_owned(),
                                        "None".to_owned()
                                    )
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
                        }),
                    ]
                }
            ))
        );

        let sample = r#"Begin Object Class=/Script/BlueprintGraph.K2Node_VariableGet Name="K2Node_VariableGet_1"
End Object
"#;
        assert_eq!(
            object(sample),
            Ok((
                "",
                Object {
                    header: ObjectHeader {
                        object_type: "Object".to_owned(),
                        header_props: vec![
                            Prop {
                                key: "Class".to_owned(),
                                value: PropValue::String(
                                    "/Script/BlueprintGraph.K2Node_VariableGet".to_owned()
                                )
                            },
                            Prop {
                                key: "Name".to_owned(),
                                value: PropValue::String("K2Node_VariableGet_1".to_owned())
                            }
                        ]
                    },
                    elements: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_object_end() {
        assert_eq!(
            object_end("End Object\n"),
            Ok((
                "",
                ObjectEnd {
                    object_type: "Object".to_owned()
                }
            ))
        );
        assert_eq!(
            object_end("End Object"),
            Ok((
                "",
                ObjectEnd {
                    object_type: "Object".to_owned()
                }
            ))
        );
        assert_eq!(
            object_end("End Level"),
            Ok((
                "",
                ObjectEnd {
                    object_type: "Level".to_owned()
                }
            ))
        );
    }

    #[test]
    fn parse_object_header() {
        assert_eq!(
            object_begin("Begin Level\n"),
            Ok((
                "",
                ObjectHeader {
                    object_type: "Level".to_owned(),
                    header_props: Vec::new()
                }
            ))
        );
        assert_eq!(
            object_begin("Begin Object Name=\"LandscapeComponent_39\"\n"),
            Ok((
                "",
                ObjectHeader {
                    object_type: "Object".to_owned(),
                    header_props: vec![Prop {
                        key: "Name".to_owned(),
                        value: PropValue::String("LandscapeComponent_39".to_owned())
                    }]
                }
            ))
        );
        assert_eq!(
            object_begin(
                "Begin Object Class=/Script/UnrealEd.EdGraphNode_Comment Name=\"K2Node_Comment_39\"\n"
            ),
            Ok((
                "",
                ObjectHeader {
                    object_type: "Object".to_owned(),
                    header_props: vec![
                        Prop {
                            key: "Class".to_owned(),
                            value: PropValue::String(
                                "/Script/UnrealEd.EdGraphNode_Comment".to_owned()
                            )
                        },
                        Prop {
                            key: "Name".to_owned(),
                            value: PropValue::String("K2Node_Comment_39".to_owned())
                        }
                    ]
                }
            ))
        );
    }

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
                vec![
                    ObjectElement::Prop(Prop {
                        key: "CustomFunctionName".to_owned(),
                        value: PropValue::String("MyEvent".to_owned())
                    }),
                    ObjectElement::Prop(Prop {
                        key: "NodePosX".to_owned(),
                        value: PropValue::Integer(512)
                    }),
                    ObjectElement::Prop(Prop {
                        key: "NodePosY".to_owned(),
                        value: PropValue::Integer(-16)
                    }),
                    ObjectElement::Prop(Prop {
                        key: "NodeGuid".to_owned(),
                        value: PropValue::Uuid(
                            Uuid::parse_str("741BFB8C4AF2854BAE60B3B660D5B625").unwrap()
                        )
                    }),
                    ObjectElement::CustomProp(CustomProp {
                        domain: "Pin".to_owned(),
                        value: CustomPropValue::Pin(vec![
                            Prop {
                                key: "PinId".to_owned(),
                                value: PropValue::Uuid(
                                    Uuid::parse_str("7CD635904148E54F000DA597BA60AB39").unwrap()
                                ),
                            },
                            Prop {
                                key: "Direction".to_owned(),
                                value: PropValue::String("EGPD_Output".to_owned())
                            },
                            Prop {
                                key: "PinType.PinSubCategoryObject".to_owned(),
                                value: PropValue::ObjectReference(
                                    "None".to_owned(),
                                    "None".to_owned()
                                )
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
                    }),
                    ObjectElement::CustomProp(CustomProp {
                        domain: "Pin".to_owned(),
                        value: CustomPropValue::Pin(vec![
                            Prop {
                                key: "PinType.PinSubCategoryObject".to_owned(),
                                value: PropValue::ObjectReference(
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
                                value: PropValue::ObjectReference(
                                    "None".to_owned(),
                                    "None".to_owned()
                                )
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
                    }),
                ]
            ))
        );
    }
}
