use std::fmt::{Debug, Formatter};
use mongodb::bson::DateTime as MDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize , Serialize)]
pub struct DateTime(MDateTime);

impl Default for DateTime{
    fn default() -> Self {
        DateTime(MDateTime::now())
    }
}
impl Debug for DateTime{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}