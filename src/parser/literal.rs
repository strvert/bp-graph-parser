use nom::{
    branch::{alt, permutation},
    bytes::complete::{escaped_transform, tag, take_until1, take_while_m_n},
    character::{
        complete,
        complete::{alphanumeric1, char, digit1, multispace0, none_of, space1},
        is_hex_digit,
    },
    combinator::{map, map_res, value},
    error::{Error, ErrorKind},
    multi::{count, separated_list0},
    sequence::delimited,
    Err, IResult,
};
use uuid::Uuid;

use super::{
    ast::{LinkedTo, Prop},
    prop::prop_kv,
};

/// A parser of string literals.
pub fn string_literal(s: &str) -> IResult<&str, String> {
    alt((
        delimited(
            char('\"'),
            escaped_transform(
                none_of("\"\\"),
                '\\',
                alt((
                    value("\\", tag("\\")),
                    value("\"", tag("\"")),
                    value("\'", tag("\'")),
                    value("\n", tag("n")),
                    value("\t", tag("t")),
                    value("\r", tag("r")),
                )),
            ),
            char('\"'),
        ),
        map(tag("\"\""), |_| "".to_string()),
    ))(s)
}

/// A parser of uuil literals.
pub fn uuid_literal(s: &str) -> IResult<&str, Uuid> {
    let r = map_res(
        take_while_m_n(32, 32, |ch| is_hex_digit(ch as u8)),
        |s: &str| Uuid::parse_str(s),
    )(s)?;
    if r.0.len() != 0 && is_hex_digit(r.0.chars().nth(0).unwrap() as u8) {
        Err(Err::Error(Error::new(s, ErrorKind::Fail)))
    } else {
        Ok(r)
    }
}

/// A parser for floating point numbers (f64).
pub fn double(s: &str) -> IResult<&str, f64> {
    map(
        permutation((complete::i64, char('.'), digit1)),
        |v: (_, _, &str)| {
            let mut r: f64 = v.0 as f64;
            let s = r / r.abs();
            for (i, c) in v.2.chars().enumerate() {
                r += (((c as i32 - 48) as f64) / (10.0 as f64).powf((i + 1) as f64)) * s;
            }
            r
        },
    )(s)
}

/// A parser of boolean.
pub fn boolean(s: &str) -> IResult<&str, bool> {
    map_res(alt((tag("True"), tag("False"))), |v| match v {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(Err::Error(Error::new(s, ErrorKind::Fail))),
    })(s)
}

/// A parser for lists of linked_object_literal.
pub fn linkedto_list_literal(s: &str) -> IResult<&str, Vec<LinkedTo>> {
    alt((
        map(tag("()"), |_| Vec::new()),
        delimited(
            char('('),
            separated_list0(
                permutation((multispace0, tag(","), multispace0)),
                linked_object_literal,
            ),
            alt((tag(",)"), tag(")"))),
        ),
    ))(s)
}

/// A parser of literals that represents a list of key/value.
pub fn kv_list_literal(s: &str) -> IResult<&str, Vec<Prop>> {
    alt((
        map(tag("()"), |_| Vec::new()),
        delimited(
            char('('),
            separated_list0(permutation((multispace0, tag(","), multispace0)), prop_kv),
            alt((tag(",)"), tag(")"))),
        ),
    ))(s)
}

/// A parser for NSLOCTEXT literals.
pub fn nsloc_text_literal(s: &str) -> IResult<&str, (String, String, String)> {
    let sp = permutation((multispace0, alt((tag(","), tag(""))), multispace0));
    map(
        permutation((
            tag("NSLOCTEXT("),
            count(permutation((string_literal, sp)), 3),
            alt((tag(",)"), tag(")"))),
        )),
        |v| {
            (
                v.1[0].0.to_string(),
                v.1[1].0.to_string(),
                v.1[2].0.to_string(),
            )
        },
    )(s)
}

/// An object reference literal parser.
pub fn object_literal(s: &str) -> IResult<&str, (String, String)> {
    alt((
        map(tag("None"), |_| ("None".to_string(), "None".to_string())),
        map(
            permutation((alphanumeric1, tag("'"), string_literal, tag("'"))),
            |v| (v.0.to_owned(), v.2),
        ),
    ))(s)
}

/// A parser of literals representing the node's connection destination.
pub fn linked_object_literal(s: &str) -> IResult<&str, LinkedTo> {
    map(permutation((take_until1(" "), space1, uuid_literal)), |v| {
        LinkedTo {
            name: v.0.to_owned(),
            uuid: v.2,
        }
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Prop, PropValue};

    #[test]
    fn parse_linkedto_list_literal() {
        assert_eq!(
            linkedto_list_literal("(K2Node_DynamicCast_46 5EE02C3B480C2249B48954B390C035D6,K2Node_CallFunction_1093 0710E8C14EFFED0DD9E024BCB29F23C3,)"),
            Ok((
                "",
                vec![
                    LinkedTo {
                        name: "K2Node_DynamicCast_46".to_string(),
                        uuid: Uuid::parse_str("5EE02C3B480C2249B48954B390C035D6").unwrap()
                    },
                    LinkedTo {
                        name: "K2Node_CallFunction_1093".to_string(),
                        uuid: Uuid::parse_str("0710E8C14EFFED0DD9E024BCB29F23C3").unwrap()
                    }
                ]
            ))
        );
        assert_eq!(
            linkedto_list_literal("(K2Node_DynamicCast_46 5EE02C3B480C2249B48954B390C035D6,K2Node_CallFunction_1093 0710E8C14EFFED0DD9E024BCB29F23C3)"),
            Ok((
                "",
                vec![
                    LinkedTo {
                        name: "K2Node_DynamicCast_46".to_string(),
                        uuid: Uuid::parse_str("5EE02C3B480C2249B48954B390C035D6").unwrap()
                    },
                    LinkedTo {
                        name: "K2Node_CallFunction_1093".to_string(),
                        uuid: Uuid::parse_str("0710E8C14EFFED0DD9E024BCB29F23C3").unwrap()
                    }
                ]
            ))
        );
    }

    #[test]
    fn parse_linked_object_literal() {
        assert_eq!(
            linked_object_literal("K2Node_InputAxisEvent_160 FCB984164512320C9D4784B5D1D93263"),
            Ok((
                "",
                LinkedTo {
                    name: "K2Node_InputAxisEvent_160".to_string(),
                    uuid: Uuid::parse_str("FCB984164512320C9D4784B5D1D93263").unwrap()
                }
            ))
        );
        assert_eq!(
            linked_object_literal("K2Node_DynamicCast_46 5EE02C3B480C2249B48954B390C035D6"),
            Ok((
                "",
                LinkedTo {
                    name: "K2Node_DynamicCast_46".to_string(),
                    uuid: Uuid::parse_str("5EE02C3B480C2249B48954B390C035D6").unwrap()
                }
            ))
        );
    }

    #[test]
    fn parse_nsloc_text_literal() {
        assert_eq!(
            nsloc_text_literal(
                r#"NSLOCTEXT("UObjectDisplayNames", "Character:CharacterMovement", "Character Movement")"#
            ),
            Ok((
                "",
                (
                    "UObjectDisplayNames".to_string(),
                    "Character:CharacterMovement".to_string(),
                    "Character Movement".to_string()
                )
            ))
        );
    }

    #[test]
    fn parse_kv_list() {
        let sample = r#"(PinId=0E1A655D4333CDC682CF73A1BB84F0FF,PinName="self",PinFriendlyName=NSLOCTEXT("K2Node", "Target", "Target"),PinToolTip="ターゲット\nGameplay Statics オブジェクト参照",PinType.PinSubCategory="",PinType.PinSubCategoryObject=Class'"/Script/Engine.GameplayStatics"',PinType.PinSubCategoryMemberReference=(),PinType.PinValueType=(PinType.ContainerType=None,PinType.bIsReference=False,PinType.PinValueType=(PinType.ContainerType=None,PinType.bIsReference=False)),DefaultObject="/Script/Engine.Default__GameplayStatics",MemberParent=BlueprintGeneratedClass'"/Game/Blueprints/PlayerCharacter.PlayerCharacter_C"',LinkedTo=(K2Node_InputAxisEvent_160 FCB984164512320C9D4784B5D1D93263,),PersistentGuid=00000000000000000000000000000000,bOrphanedPin=True,)"#;
        assert_eq!(
            kv_list_literal(sample),
            Ok((
                "",
                vec![
                    Prop {
                        key: "PinId".to_owned(),
                        value: PropValue::Uuid(
                            Uuid::parse_str("0E1A655D4333CDC682CF73A1BB84F0FF").unwrap()
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
                        key: "PinToolTip".to_owned(),
                        value: PropValue::String(
                            "ターゲット\nGameplay Statics オブジェクト参照".to_owned()
                        )
                    },
                    Prop {
                        key: "PinType.PinSubCategory".to_owned(),
                        value: PropValue::String("".to_owned())
                    },
                    Prop {
                        key: "PinType.PinSubCategoryObject".to_owned(),
                        value: PropValue::ObjectReference(
                            "Class".to_owned(),
                            "/Script/Engine.GameplayStatics".to_owned()
                        )
                    },
                    Prop {
                        key: "PinType.PinSubCategoryMemberReference".to_owned(),
                        value: PropValue::PropList(Vec::new())
                    },
                    Prop {
                        key: "PinType.PinValueType".to_owned(),
                        value: PropValue::PropList(vec![
                            Prop {
                                key: "PinType.ContainerType".to_owned(),
                                value: PropValue::ObjectReference(
                                    "None".to_owned(),
                                    "None".to_owned()
                                )
                            },
                            Prop {
                                key: "PinType.bIsReference".to_owned(),
                                value: PropValue::Boolean(false)
                            },
                            Prop {
                                key: "PinType.PinValueType".to_owned(),
                                value: PropValue::PropList(vec![
                                    Prop {
                                        key: "PinType.ContainerType".to_owned(),
                                        value: PropValue::ObjectReference(
                                            "None".to_owned(),
                                            "None".to_owned()
                                        )
                                    },
                                    Prop {
                                        key: "PinType.bIsReference".to_owned(),
                                        value: PropValue::Boolean(false)
                                    },
                                ])
                            },
                        ])
                    },
                    Prop {
                        key: "DefaultObject".to_owned(),
                        value: PropValue::String(
                            "/Script/Engine.Default__GameplayStatics".to_owned()
                        )
                    },
                    Prop {
                        key: "MemberParent".to_owned(),
                        value: PropValue::ObjectReference(
                            "BlueprintGeneratedClass".to_owned(),
                            "/Game/Blueprints/PlayerCharacter.PlayerCharacter_C".to_owned()
                        )
                    },
                    Prop {
                        key: "LinkedTo".to_owned(),
                        value: PropValue::LinkedToList(vec![LinkedTo {
                            name: "K2Node_InputAxisEvent_160".to_owned(),
                            uuid: Uuid::parse_str("FCB984164512320C9D4784B5D1D93263").unwrap()
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
                        value: PropValue::Boolean(true)
                    },
                ]
            ))
        );
        assert_eq!(kv_list_literal(r#"()"#), Ok(("", vec![])));
        assert_eq!(
            kv_list_literal(r#"(MemberName="AddControllerYawInput",bSelfContext=True)"#),
            Ok((
                "",
                vec![
                    Prop {
                        key: "MemberName".to_owned(),
                        value: PropValue::String("AddControllerYawInput".to_owned())
                    },
                    Prop {
                        key: "bSelfContext".to_owned(),
                        value: PropValue::Boolean(true)
                    }
                ]
            ))
        );
    }

    #[test]
    fn parse_string_literal() {
        assert_eq!(
            string_literal(r#""hoge fuga""#),
            Ok(("", "hoge fuga".to_string()))
        );
        assert_eq!(
            string_literal(r#""hoge\n \r \t\\ \' f\"uga""#),
            Ok(("", "hoge\n \r \t\\ ' f\"uga".to_string()))
        );
        assert_eq!(
            string_literal(r#""hoge\n \r \t\\ \' f\"uga" piyo"#),
            Ok((" piyo", "hoge\n \r \t\\ ' f\"uga".to_string()))
        );
    }

    #[test]
    fn parse_uuid_literal() {
        assert_eq!(
            uuid_literal("914EEC5D4C41B6D45E6DB79302BCC5BA"),
            Ok((
                "",
                Uuid::parse_str("914EEC5D4C41B6D45E6DB79302BCC5BA").unwrap()
            ))
        );
        assert_eq!(
            uuid_literal("914EEC5D4C41B6D45E6DB79302BCC5BAFFF"),
            Err(Err::Error(Error::new(
                "914EEC5D4C41B6D45E6DB79302BCC5BAFFF",
                ErrorKind::Fail
            )))
        );
        assert_ne!(
            uuid_literal("49B87A28472C0D6908043885A7306D2A"),
            Ok((
                "",
                Uuid::parse_str("914EEC5D4C41B6D45E6DB79302BCC5BA").unwrap()
            ))
        );
        assert_ne!(
            uuid_literal("49B87A28472C0D6908043885A7306D2A,FFF"),
            Ok((
                ",FFF",
                Uuid::parse_str("914EEC5D4C41B6D45E6DB79302BCC5BA").unwrap()
            ))
        );
        assert_eq!(
            uuid_literal("MMMMMMMMMMMMMMMMMMMMMMMMMMMMC5BA"),
            Err(Err::Error(Error::new(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMC5BA",
                ErrorKind::TakeWhileMN
            )))
        );
        assert_eq!(
            uuid_literal("49B87A6D2A"),
            Err(Err::Error(Error::new("49B87A6D2A", ErrorKind::TakeWhileMN)))
        );
    }
}
