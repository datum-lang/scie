use serde::{Deserialize, Deserializer};

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<u8>::deserialize(deserializer)? {
        None => Ok(None),
        Some(value) => match value {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            other => {
                println!("{:?}", other);
                return Ok(Some(false));
            }
        },
    }
}
