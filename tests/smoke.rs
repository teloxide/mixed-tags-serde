use serde::Serialize;

#[mixed_tags_serde::mixed_tags]
#[derive(Serialize)]
enum MyEnum {
    One(i32),
    Two(u32),
    Three { x: i32, y: i32 }
}