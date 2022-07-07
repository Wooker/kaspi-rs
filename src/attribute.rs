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
#[serde(untagged)]
pub enum Value {
    String(String),
    Boolean(bool)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeValue {
    pub code: String,
    pub value: Value,
}

mod tests {
    #[test]
    fn attribute_to_json() {
        use super::{AttributeValue, Value};

        let attribute = AttributeValue {
            code: String::from("ASD"),
            value: Value::String(String::from("ASD"))
        };

        let json = serde_json::to_value(attribute).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "code": "ASD",
                "value": "ASD"
            })
        );
    }
}
