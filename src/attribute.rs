use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct KaspiCategoryAttribute {
    pub code: String,
    pub r#type: String,
    pub multiValued: bool,
    pub mandatory: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeValue {
    pub code: String,
    pub name: String,
}
