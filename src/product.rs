use serde::{Deserialize, Serialize};
use crate::{
    attribute::AttributeValue,
    upload_result::UploadResult,
};
use std::fmt;

/*
#[derive(Error, Debug)]
pub enum ProductError {
    #[error("Could not parse input")]
    Parsing(#[from] FromStr::Error),
    CategoryFetch,
    Images,
}
*/

#[derive(Serialize, Deserialize, Debug)]
struct ProductImage {
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    sku: String,
    title: String,
    brand: String,
    category: String,
    description: String,
    attributes: Vec<AttributeValue>,
    images: Vec<ProductImage>,
    code: String,
    status: String,
    result: Option<UploadResult>,
}

impl Product {
    pub fn sku(&self) -> String {
        self.sku.clone()
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
