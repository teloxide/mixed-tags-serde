#[mixed_tags_serde::mixed_tags(Serialize, Deserialize)]
#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
enum MyEnum {
    #[serde(rename = "one")]
    #[tagged]
    One(i32),

    #[serde(rename = "two")]
    #[tagged]
    Two(u32),

    // #[tagged]
    Three {
        x: i32,
        y: i32,
    },
}

#[test]
fn test_se() {
    let origin = MyEnum::One(-123);
    let expected = r#"{"One":-123}"#;
    let actual = serde_json::to_string(&origin).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_de() {
    let origin = r#"{"Two":567}"#;
    let expected = MyEnum::Two(567);
    let actual: MyEnum = serde_json::from_str(origin).unwrap();
    assert_eq!(actual, expected);
}
