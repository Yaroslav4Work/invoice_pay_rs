use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PointType {
    Offline,
    Online,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PointOfSale {
    pub id: Option<String>,
    pub name: String,
    pub address: String,
    pub alias: String,
    pub mail: String,
    pub phone: String,
    #[serde(rename = "type")]
    pub point_type: PointType,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TerminalType {
    Dynamical,
    Statical,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
    pub id: Option<String>,
    pub link: Option<String>,
    pub name: String,
    pub alias: Option<String>,
    pub description: String,
    #[serde(rename = "type")]
    pub terminal_type: TerminalType,
    pub default_price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    RUB,
    EUR,
    USD,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub currency: Currency,
    pub amount: f32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentDtoReceipt {
    pub name: String,
    pub price: f32,
    pub discount: f32,
    pub result_price: f32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CreatePaymentDtoSettings {
    pub terminal_id: String,
    pub success_url: String,
    pub fail_url: String,
    pub recur_exp: Option<String>,
    pub recur_freq: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CreatePaymentDtoTransactionType {
    Once = 1,
    Recurrent = 4,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentDto {
    pub order: Order,
    pub settings: CreatePaymentDtoSettings,
    pub custom_parameters: Option<HashMap<String, String>>,
    pub receipt: Vec<CreatePaymentDtoReceipt>,
    pub phone: Option<String>,
    pub mail: Option<String>,
    #[serde(rename = "trtype")]
    pub transaction_type: CreatePaymentDtoTransactionType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    payment_method_type: Option<String>,
    terminal_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PaymentResponseDto {
    pub id: String,
    pub order: Order,
    pub status: String,
    pub status_description: String,
    pub payment_method: PaymentMethod,
    pub custom_parameters: Option<HashMap<String, String>>,
    pub create_date: String,
    pub update_date: Option<String>,
    pub expire_date: Option<String>,
    pub payment_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub error: u8,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiValidationErrorResponse {
    pub error: u8,
    pub description: String,
    pub additions: HashMap<String, Vec<String>>,
}