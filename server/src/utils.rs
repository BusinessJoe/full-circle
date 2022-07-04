use serde::{Deserialize, Deserializer};

pub fn make_hint(answer: &str) -> String {
    answer
        .chars()
        .map(|c| if c.is_alphanumeric() { '_' } else { c })
        .collect()
}

pub fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

pub fn deserialize_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|string| string.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hint() {
        assert_eq!(make_hint("hello world"), String::from("_____ _____"));
    }

    fn test_hint_numbers_punctuation() {
        assert_eq!(
            make_hint(r#"ab2%^' "def-+"#),
            String::from(r#"___%^' "___-+"#)
        );
    }
}
