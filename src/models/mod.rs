use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Slab {
    pub id: Option<i32>,
    pub owner: String,
    pub personal_collection: i32,
    pub card_name: String,
    pub card_number: String,
    pub set_name: String,
    pub tcg: String,
    pub language: String,
    pub slab_case: Option<String>,
    pub grading_company: String,
    pub grade: f64,
    pub cert_number: String,
    pub price: Option<f64>,
    pub sold: i32,
    pub sold_value: Option<f64>,
    pub postage_and_fees: Option<f64>,
    pub date_sold: Option<String>,
    pub notes: String,
    pub image_url: Option<String>,
    pub ace_label_url: Option<String>,
    pub listing_url: Option<String>,
    pub updated_at: Option<String>,
}
