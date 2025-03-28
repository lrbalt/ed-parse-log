pub mod duration_as_secs {
    use chrono::Duration;
    use serde::{self, Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let secs = duration.num_seconds();
        serializer.serialize_i64(secs)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = i64::deserialize(deserializer)?;
        match Duration::try_seconds(secs) {
            Some(d) => Ok(d),
            None => Err(D::Error::custom("Cannot parse duration from seconds")),
        }
    }
}