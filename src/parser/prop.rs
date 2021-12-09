use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_till1, take_while1},
    character::{
        complete,
        complete::{alphanumeric1, char, multispace0, space1},
        is_newline, is_space,
    },
    combinator::map,
    error::{Error, ErrorKind},
    Err, IResult,
};

use super::{
    ast::{CustomProp, CustomPropValue, Prop, PropValue, LinkedTo},
    literal::{
        boolean, double, kv_list_literal, linkedto_list_literal, nsloc_text_literal,
        object_literal, string_literal, uuid_literal,
    },
};

pub fn prop_value(s: &str) -> IResult<&str, PropValue> {
    alt((
        map(boolean, |v| PropValue::Boolean(v)),
        map(uuid_literal, |v| PropValue::Uuid(v)),
        map(string_literal, |v| PropValue::String(v)),
        map(nsloc_text_literal, |v| PropValue::NslocText(v.0, v.1, v.2)),
        map(object_literal, |v| PropValue::ObjectReference(v.0, v.1)),
        map(double, |v| PropValue::Double(v)),
        map(complete::i64, |v| PropValue::Integer(v)),
        map(kv_list_literal, |v| PropValue::PropList(v)),
        map(linkedto_list_literal, |v| PropValue::LinkedToList(v)),
    ))(s)
}

pub fn prop_kv(s: &str) -> IResult<&str, Prop> {
    let (s, (_, key, _, _, _, value)) = permutation((
        multispace0,
        take_till1(|c: char| is_space(c as u8) || c == '='),
        multispace0,
        char('='),
        multispace0,
        prop_value,
    ))(s)?;
    Ok((
        s,
        Prop {
            key: key.to_string(),
            value,
        },
    ))
}

pub fn prop_custom_props(s: &str) -> IResult<&str, CustomProp> {
    let (ns, (_, _, name, _)) =
        permutation((tag("CustomProperties"), space1, alphanumeric1, space1))(s)?;
    let (ns, prop_code) = take_while1(|c| !is_newline(c as u8))(ns)?;
    let custom_prop = match name {
        "Pin" => map(kv_list_literal, |v| CustomProp {
            domain: name.to_string(),
            value: CustomPropValue::Pin(v),
        })(prop_code)?,
        _ => return Err(Err::Error(Error::new(s, ErrorKind::Fail))),
    };
    Ok((ns, custom_prop.1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use uuid::Uuid;

    #[test]
    fn parse_custom_props() {
        let sample = r#"CustomProperties Pin (PinId=F6D0DA4A4AA531533341018A20422309,PinName="self",PinFriendlyName=NSLOCTEXT("K2Node", "Target", "Target"),PinType.PinCategory="object",PinType.PinSubCategory="",PinType.PinSubCategoryObject=Class'"/Script/UMG.Button"',PinType.PinSubCategoryMemberReference=(),PinType.ContainerType=None,PinType.bIsReference=True,LinkedTo=(K2Node_VariableGet_17 570BAD4542CBB0285413EEAB4F6DBDDA,),PersistentGuid=00000000000000000000000000000000,bOrphanedPin=False,)"#;
        assert_eq!(
            prop_custom_props(sample),
            Ok((
                "",
                CustomProp {
                    domain: "Pin".to_string(),
                    value: CustomPropValue::Pin(vec![
                        Prop {
                            key: "PinId".to_owned(),
                            value: PropValue::Uuid(
                                Uuid::parse_str("F6D0DA4A4AA531533341018A20422309").unwrap()
                            )
                        },
                        Prop {
                            key: "PinName".to_owned(),
                            value: PropValue::String("self".to_owned())
                        },
                        Prop {
                            key: "PinFriendlyName".to_owned(),
                            value: PropValue::NslocText(
                                "K2Node".to_owned(),
                                "Target".to_owned(),
                                "Target".to_owned()
                            )
                        },
                        Prop {
                            key: "PinType.PinCategory".to_owned(),
                            value: PropValue::String("object".to_owned())
                        },
                        Prop {
                            key: "PinType.PinSubCategory".to_owned(),
                            value: PropValue::String("".to_owned())
                        },
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
                            value: PropValue::ObjectReference("None".to_owned(), "None".to_owned())
                        },
                        Prop {
                            key: "PinType.bIsReference".to_owned(),
                            value: PropValue::Boolean(true)
                        },
                        Prop {
                            key: "LinkedTo".to_owned(),
                            value: PropValue::LinkedToList(vec![LinkedTo {
                                name: "K2Node_VariableGet_17".to_owned(),
                                uuid: Uuid::parse_str("570BAD4542CBB0285413EEAB4F6DBDDA").unwrap()
                            }])
                        },
                        Prop {
                            key: "PersistentGuid".to_owned(),
                            value: PropValue::Uuid(
                                Uuid::parse_str("00000000000000000000000000000000").unwrap()
                            )
                        },
                        Prop {
                            key: "bOrphanedPin".to_owned(),
                            value: PropValue::Boolean(false)
                        },
                    ])
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_integer() {
        assert_eq!(
            prop_kv("NodePosX=1088"),
            Ok((
                "",
                Prop {
                    key: "NodePosX".to_string(),
                    value: PropValue::Integer(1088)
                }
            ))
        );
        assert_eq!(
            prop_kv("Node.PosX=1088"),
            Ok((
                "",
                Prop {
                    key: "Node.PosX".to_string(),
                    value: PropValue::Integer(1088)
                }
            ))
        );
        assert_eq!(
            prop_kv("NodePosY=-23088"),
            Ok((
                "",
                Prop {
                    key: "NodePosY".to_string(),
                    value: PropValue::Integer(-23088)
                }
            ))
        );
        assert_eq!(
            prop_kv(" \t  NodePosY  = \t-192314 \t  "),
            Ok((
                " \t  ",
                Prop {
                    key: "NodePosY".to_string(),
                    value: PropValue::Integer(-192314)
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_boolean() {
        assert_eq!(
            prop_kv("bSelfContext=True"),
            Ok((
                "",
                Prop {
                    key: "bSelfContext".to_string(),
                    value: PropValue::Boolean(true)
                }
            ))
        );
        assert_eq!(
            prop_kv("bSelfContext=False"),
            Ok((
                "",
                Prop {
                    key: "bSelfContext".to_string(),
                    value: PropValue::Boolean(false)
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_double() {
        let r = prop_kv("X=-560.123400").unwrap();
        assert_eq!(r.0, "");
        assert_eq!(r.1.key, "X".to_owned());
        assert_approx_eq!(
            match r.1.value {
                PropValue::Double(v) => v,
                _ => 0.0,
            },
            -560.123400
        );
    }

    #[test]
    fn parse_prop_kv_uuid() {
        assert_eq!(
            prop_kv("NodeGuid=72D31250462697EE90B27CBFC0957A6D"),
            Ok((
                "",
                Prop {
                    key: "NodeGuid".to_string(),
                    value: PropValue::Uuid(
                        Uuid::parse_str("72D31250462697EE90B27CBFC0957A6D").unwrap()
                    )
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_string() {
        assert_eq!(
            prop_kv("NodeComment=\"\""),
            Ok((
                "",
                Prop {
                    key: "NodeComment".to_string(),
                    value: PropValue::String("".to_string())
                }
            ))
        );
        assert_eq!(
            prop_kv("NodeComment=\"Mouse input\""),
            Ok((
                "",
                Prop {
                    key: "NodeComment".to_string(),
                    value: PropValue::String("Mouse input".to_string())
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_class() {
        assert_eq!(
            prop_kv(r#"PinType.PinSubCategoryObject=Class'"/Script/Engine.GameplayStatics"'"#),
            Ok((
                "",
                Prop {
                    key: "PinType.PinSubCategoryObject".to_string(),
                    value: PropValue::ObjectReference(
                        "Class".to_string(),
                        "/Script/Engine.GameplayStatics".to_string()
                    )
                }
            ))
        );
        assert_eq!(
            prop_kv(r#"PinType.PinSubCategoryObject=None"#),
            Ok((
                "",
                Prop {
                    key: "PinType.PinSubCategoryObject".to_string(),
                    value: PropValue::ObjectReference("None".to_string(), "None".to_string())
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_nsloctext() {
        assert_eq!(
            prop_kv(r#"PinFriendlyName=NSLOCTEXT("K2Node", "Target", "Target")"#),
            Ok((
                "",
                Prop {
                    key: "PinFriendlyName".to_string(),
                    value: PropValue::NslocText(
                        "K2Node".to_string(),
                        "Target".to_string(),
                        "Target".to_string()
                    )
                }
            ))
        );
        assert_eq!(
            prop_kv("NodeComment=\"Mouse input\""),
            Ok((
                "",
                Prop {
                    key: "NodeComment".to_string(),
                    value: PropValue::String("Mouse input".to_string())
                }
            ))
        );
    }

    #[test]
    fn parse_prop_kv_proplist() {
        assert_eq!(
            prop_kv(r#"VariableReference=(MemberName="CharacterMovement",bSelfContext=True)"#),
            Ok((
                "",
                Prop {
                    key: "VariableReference".to_string(),
                    value: PropValue::PropList(vec![
                        Prop {
                            key: "MemberName".to_owned(),
                            value: PropValue::String("CharacterMovement".to_owned())
                        },
                        Prop {
                            key: "bSelfContext".to_owned(),
                            value: PropValue::Boolean(true)
                        }
                    ])
                }
            ))
        );
    }
}
