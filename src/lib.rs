mod visitor;

use std::slice;

use serde::{
    Deserialize, Serialize,
    ser::{Serializer, SerializeStruct},
    de::Deserializer,
};

use visitor::TaggedVisitor;

pub fn tagged_de<'de, T, D>(des: D, ty: &'static str, tag: &'static &'static str) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    des.deserialize_struct(ty, slice::from_ref(&tag), TaggedVisitor::new(tag))
}

pub fn tagged_ser<T, S>(val: &T, ser: S, ty: &'static str, tag: &'static str) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut sv = ser.serialize_struct(ty, 1)?;
    sv.serialize_field(tag, val)?;
    sv.end()
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    #[serde(untagged)]
    enum Enum {
        #[serde(with = "tagged_i32")]
        TaggedI32(i32),
        #[serde(with = "tagged_string")]
        TaggedString(String),
        Untagged {
            x: i32,
            y: i32
        }
    }

    mod tagged_i32 {
        use serde::{Deserializer, Serializer};

        use crate::{tagged_ser, tagged_de};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
            where
                D: Deserializer<'de>
        {
            tagged_de(deserializer, "TaggedI32", &"tagged_i32")
        }

        pub fn serialize<S>(val: &i32, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer
        {
            tagged_ser(val, serializer, "TaggedI32", "tagged_i32")
        }
    }

    mod tagged_string {
        use serde::{Deserializer, Serializer};

        use crate::{tagged_ser, tagged_de};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
            where
                D: Deserializer<'de>
        {
            tagged_de(deserializer, "TaggedString", &"tagged_string")
        }

        pub fn serialize<S>(val: &String, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer
        {
            tagged_ser(val, serializer, "TaggedString", "tagged_string")
        }
    }

    #[test]
    fn ser_tagged_i32() {
        assert_eq!(
            serde_json::to_string(&Enum::TaggedI32(123)).unwrap(),
            String::from(r#"{"tagged_i32":123}"#)
        );
    }

    #[test]
    fn ser_tagged_string() {
        assert_eq!(
            serde_json::to_string(&Enum::TaggedString(String::from("str"))).unwrap(),
            String::from(r#"{"tagged_string":"str"}"#)
        );
    }

    #[test]
    fn ser_untagged() {
        assert_eq!(
            serde_json::to_string(&Enum::Untagged { x: 42, y: 17 }).unwrap(),
            String::from(r#"{"x":42,"y":17}"#)
        );
    }

    #[test]
    fn de_tagged_i32() {
        assert_eq!(
            serde_json::from_str::<Enum>(r#"{"tagged_i32":123}"#).unwrap(),
            Enum::TaggedI32(123)
        );
    }

    #[test]
    fn de_tagged_string() {
        assert_eq!(
            serde_json::from_str::<Enum>(r#"{"tagged_string":"str"}"#).unwrap(),
            Enum::TaggedString(String::from("str"))
        );
    }

    #[test]
    fn de_untagged() {
        assert_eq!(
            serde_json::from_str::<Enum>(r#"{"x":42,"y":17}"#).unwrap(),
            Enum::Untagged { x: 42, y: 17 }
        );
    }
}
