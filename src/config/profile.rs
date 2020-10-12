//this one is lifted from https://github.com/serde-rs/serde/issues/936
use super::Profile;

use std::collections::HashMap;

use serde::de::{Deserializer, Visitor, SeqAccess};

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Profile>, D::Error>
    where D: Deserializer<'de>
{
    struct ProfileVisitor;

    impl<'de> Visitor<'de> for ProfileVisitor {
        type Value = HashMap<String, Profile>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of profiles")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<HashMap<String, Profile>, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut map = HashMap::with_capacity(seq.size_hint().unwrap_or(0));

            while let Some(profile) = seq.next_element::<Profile>()? {
                map.insert(profile.name.to_owned(), profile);
            }

            Ok(map)
        }
    }

    deserializer.deserialize_seq(ProfileVisitor)
}
