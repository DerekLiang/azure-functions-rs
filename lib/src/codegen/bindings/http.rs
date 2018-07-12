use serde::{ser::SerializeMap, Serialize, Serializer};

#[doc(hidden)]
pub struct Http {
    pub name: &'static str,
}

// TODO: when https://github.com/serde-rs/serde/issues/760 is resolved, remove implementation in favor of custom Serialize derive
// The fix would allow us to set the constant `type` and `direction` entries rather than having to emit them manually.
impl Serialize for Http {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("name", self.name)?;
        map.serialize_entry("type", "http")?;
        map.serialize_entry("direction", "out")?;

        map.end()
    }
}
