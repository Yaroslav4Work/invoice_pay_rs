use crate::error::Error;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
    pub alias: String,
    pub description: String,
    #[serde(rename = "type")]
    pub terminal_type: TerminalType,
    pub default_price: f32,
    pub point_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentDtoSettings {
    pub terminal_id: String,
    pub success_url: String,
    pub fail_url: String,
    pub recur_exp: Option<String>,
    pub recur_freq: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct GetByIdentityDto {
    id: Option<String>,
    alias: Option<String>,
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

pub struct InvoicePay {
    api_key: String,
    client: Client,
}

impl InvoicePay {
    fn new(api_key: String) -> InvoicePay {
        InvoicePay {
            api_key,
            client: Client::new(),
        }
    }

    fn get_base_url() -> &'static str {
        "https://api.invoice.su/api/v2"
    }

    // Point of sale

    async fn get_point_of_sale(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<PointOfSale, Error> {
        if id.is_none() && alias.is_none() {
            return Err(Error::new(
                "At least one of the id or alias parameters must be",
            ));
        }

        let data = GetByIdentityDto { id, alias };

        let req = self
            .client
            .post(format!("{}/GetPointOfSale", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&data);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /GetPointOfSale"))?
            .json::<PointOfSale>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /GetPointOfSale"))?;

        Ok(res)
    }

    async fn create_point_of_sale(&self, point_of_sale: PointOfSale) -> Result<PointOfSale, Error> {
        let req = self
            .client
            .post(format!("{}/CreatePointOfSale", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&point_of_sale);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /CreatePointOfSale"))?
            .json::<PointOfSale>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /CreatePointOfSale"))?;

        Ok(res)
    }

    // Terminal

    async fn get_terminal(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<Terminal, Error> {
        if id.is_none() && alias.is_none() {
            return Err(Error::new(
                "At least one of the id or alias parameters must be",
            ));
        }

        let data = GetByIdentityDto { id, alias };

        let req = self
            .client
            .post(format!("{}/GetTerminal", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&data);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /GetTerminal"))?
            .json::<Terminal>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /GetTerminal"))?;

        Ok(res)
    }

    async fn create_terminal(&self, terminal: Terminal) -> Result<Terminal, Error> {
        let req = self
            .client
            .post(format!("{}/CreateTerminal", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&terminal);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /CreateTerminal"))?
            .json::<Terminal>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /CreateTerminal"))?;

        Ok(res)
    }

    // Payment
    
    async fn get_payment(
        &self,
        id: String,
    ) -> Result<PaymentResponseDto, Error> {
        let data = GetByIdentityDto { id: Some(id), alias: None };

        let req = self
            .client
            .post(format!("{}/GetPayment", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&data);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /GetPayment"))?
            .json::<PaymentResponseDto>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /GetPayment"))?;

        Ok(res)
    }
    
    async fn create_payment(&self, payment_dto: CreatePaymentDto) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/CreatePayment", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&payment_dto);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /CreatePayment"))?
            .json::<PaymentResponseDto>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /CreatePayment"))?;

        Ok(res)
    }
}
