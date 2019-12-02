// #[allow(unused_imports)]
// use serde::Serialize;

#[mixed_tag_serde::mixed_tags(Serialize, Deserialize)]
#[derive(serde::Serialize)]
enum MyEnum {
    #[serde(rename = "one")]
    #[tagged]
    One(i32),

    #[serde(rename = "two")]
    #[tagged]
    Two(u32),

    Three { x: i32, y: i32 }
}

#[test]
fn test() {
    let origin = MyEnum::One(-123);
    let expected = r#"{"One":-123}"#;
    let actual = serde_json::to_string(&origin).unwrap();
    assert_eq!(actual, expected);
}
