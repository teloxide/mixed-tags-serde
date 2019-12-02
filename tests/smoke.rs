// #[allow(unused_imports)]
// use serde::Serialize;
/*#![feature(trace_macros)]

trace_macros!(true);*/

#[allow(dead_code)]
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
