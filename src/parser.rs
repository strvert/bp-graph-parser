use nom::{
    branch::{alt, permutation},
    bytes::complete::{escaped_transform, tag, take_till, take_till1, take_until1, take_while_m_n},
    character::{
        complete,
        complete::{alphanumeric1, char, digit1, multispace0, none_of, one_of},
        is_hex_digit, is_space,
    },
    combinator::{map, map_res, value},
    error::{Error, ErrorKind},
    multi::{count, separated_list0},
    sequence::delimited,
    Err, IResult,
};
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
    key: String,
    value: PropValue,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    props: Vec<Prop>,
}

fn string_literal(s: &str) -> IResult<&str, String> {
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

fn uuid_literal(s: &str) -> IResult<&str, Uuid> {
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

fn kv_list_literal(s: &str) -> IResult<&str, Vec<Prop>> {
    alt((
        map(tag("()"), |_| Vec::new()),
        delimited(
            char('('),
            separated_list0(permutation((multispace0, tag(","), multispace0)), prop_kv),
            alt((tag(",)"), tag(")"))),
        ),
    ))(s)
}

fn nsloc_text_literal(s: &str) -> IResult<&str, (String, String, String)> {
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

fn object_literal(s: &str) -> IResult<&str, (String, String)> {
    alt((
        map(tag("None"), |v| ("None".to_string(), "None".to_string())),
        map(
            permutation((alphanumeric1, tag("'"), string_literal, tag("'"))),
            |v| (v.0.to_owned(), v.2),
        ),
    ))(s)
}

fn double(s: &str) -> IResult<&str, f64> {
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

fn boolean(s: &str) -> IResult<&str, bool> {
    map_res(alt((tag("True"), tag("False"))), |v| match v {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(Err::Error(Error::new(s, ErrorKind::Fail))),
    })(s)
}

fn prop_value(s: &str) -> IResult<&str, PropValue> {
    alt((
        map(boolean, |v| PropValue::Boolean(v)),
        map(uuid_literal, |v| PropValue::Uuid(v)),
        map(string_literal, |v| PropValue::String(v)),
        map(nsloc_text_literal, |v| PropValue::NslocText(v.0, v.1, v.2)),
        map(object_literal, |v| PropValue::Object(v.0, v.1)),
        map(double, |v| PropValue::Double(v)),
        map(complete::i64, |v| PropValue::Integer(v)),
        map(kv_list_literal, |v| PropValue::PropList(v)),
    ))(s)
}

fn prop_kv(s: &str) -> IResult<&str, Prop> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use nom::error::ErrorKind;

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
        let sample = r#"(PinId=0E1A655D4333CDC682CF73A1BB84F0FF,PinName="self",PinFriendlyName=NSLOCTEXT("K2Node", "Target", "Target"),PinToolTip="ターゲット\nGameplay Statics オブジェクト参照",PinType.PinSubCategory="",PinType.PinSubCategoryObject=Class'"/Script/Engine.GameplayStatics"',PinType.PinSubCategoryMemberReference=(),PinType.PinValueType=(PinType.ContainerType=None,PinType.bIsReference=False,PinType.PinValueType=(PinType.ContainerType=None,PinType.bIsReference=False)),DefaultObject="/Script/Engine.Default__GameplayStatics",MemberParent=BlueprintGeneratedClass'"/Game/Blueprints/PlayerCharacter.PlayerCharacter_C"',PersistentGuid=00000000000000000000000000000000,bOrphanedPin=True,)"#;
        println!("{:#?}", kv_list_literal(sample));
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
                        value: PropValue::Object(
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
                                value: PropValue::Object("None".to_owned(), "None".to_owned())
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
                                        value: PropValue::Object(
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
                        value: PropValue::Object(
                            "BlueprintGeneratedClass".to_owned(),
                            "/Game/Blueprints/PlayerCharacter.PlayerCharacter_C".to_owned()
                        )
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
                    value: PropValue::Object(
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
                    value: PropValue::Object("None".to_string(), "None".to_string())
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
