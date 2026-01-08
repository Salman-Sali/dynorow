use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(bound = "T: serde::Serialize +  serde::de::DeserializeOwned")]
pub struct SerdeField<T: serde::de::DeserializeOwned> where T: 'static + Clone {
    pub value: T
}

impl<T> FromStr for SerdeField<T> where T: 'static + Clone + serde::de::DeserializeOwned + serde::Serialize
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| format!("{e:?}"))
    }
}

impl<T> SerdeField<T> where T: 'static + Clone + serde::de::DeserializeOwned + serde::Serialize
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}