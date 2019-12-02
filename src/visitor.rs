use std::marker::PhantomData;

use serde::{Deserialize, de::{Visitor, MapAccess, Error}};

pub(crate) struct TaggedVisitor<T> {
    tag: &'static str,
    marker: PhantomData<T>,
}

impl<T> TaggedVisitor<T> {
    pub(crate) fn new(tag: &'static str) -> Self {
        Self { tag, marker: PhantomData::<T> }
    }
}

impl<'de, T> Visitor<'de> for TaggedVisitor<T>
where
    T: Deserialize<'de>
{
    type Value = T;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "") // TODO: what we expect?
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let entry = map
            .next_entry::<&'de str, T>()?
            .ok_or_else(|| {
                let ty = std::any::type_name::<T>();
                let message = format!(
                    r#"pair of key "{tag}" and value of type {T}"#, // TODO: better message?
                    tag = self.tag,
                    T = ty
                );
                A::Error::custom(message)
            })?;


        let (tag, value) = entry;
        if tag == self.tag {
            Ok(value)
        } else {
            Err(A::Error::custom(format!(r#"key "{tag}""#, tag = self.tag)))
        }
    }
}