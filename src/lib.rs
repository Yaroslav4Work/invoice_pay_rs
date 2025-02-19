use crate::error::Error;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::ops::Add;

mod error;

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

pub struct InvoicePay {
    api_key: String,
    client: Client,
}

impl InvoicePay {
    fn new(login: String, api_key: String) -> InvoicePay {
        InvoicePay {
            api_key: STANDARD.encode(login.add(":").add(&api_key)),
            client: Client::new(),
        }
    }

    fn get_base_url() -> &'static str {
        "https://api.invoice.su/api/v2"
    }

    fn serialize_identity_dto(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<String, Error> {
        let mut data: HashMap<&str, String> = HashMap::new();

        if id.is_some() {
            data.insert("id", id.unwrap());
        } else if alias.is_some() {
            data.insert("alias", alias.unwrap());
        } else {
            return Err(Error::new(
                "At least one of the id or alias parameters must be",
            ));
        }

        Ok(
            serde_json::to_string(&data)
                .map_err(|_| Error::new("Cannot serialize identity dto"))?,
        )
    }

    // Point of sale

    async fn get_point_of_sale(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<PointOfSale, Error> {
        let req = self
            .client
            .post(format!("{}/GetPointOfSale", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(id, alias)?);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /GetPointOfSale"))?
            .text()
            .await
            .map_err(|_| Error::new("Cannot convert response from /GetPointOfSale"))?;

        let res = serde_json::from_str(&res).map_err(|e| {
            Error::new(&format!(
                "Cannot parse response from /GetPointOfSale: {}",
                e
            ))
        })?;

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
            .text()
            .await
            .map_err(|_| Error::new("Cannot convert Response String to UTF-8 &str"))?;

        let res = serde_json::from_str::<PointOfSale>(&res)
            .map_err(|_| Error::new("Cannot parse response from response"))?;

        Ok(res)
    }

    // Terminal

    async fn get_terminal(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<Terminal, Error> {
        let req = self
            .client
            .post(format!("{}/GetTerminal", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(id, alias)?);

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
            .body(serde_json::to_string(&terminal).unwrap());

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /CreateTerminal"))?
            .text()
            .await
            .map_err(|_| Error::new("Cannot parse response from /CreateTerminal"))?;

        let res = serde_json::from_str(&res)
            .map_err(|_| Error::new("Cannot parse response from /CreateTerminal"))?;

        Ok(res)
    }

    // Payment

    async fn get_payment(&self, id: String) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/GetPayment", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(Some(id), None)?);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /GetPayment"))?
            .json::<PaymentResponseDto>()
            .await
            .map_err(|_| Error::new("Cannot parse response from /GetPayment"))?;

        Ok(res)
    }

    async fn create_payment(
        &self,
        payment_dto: CreatePaymentDto,
    ) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/CreatePayment", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(serde_json::to_string(&payment_dto).unwrap());

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /CreatePayment"))?
            .text()
            .await
            .map_err(|_| Error::new("Cannot convert response from /CreatePayment"))?;

        let res = serde_json::from_str(&res)
            .map_err(|_| Error::new("Cannot parse response from /CreatePayment"))?;

        Ok(res)
    }

    async fn cancel_payment(&self, id: String) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/ClosePayment", InvoicePay::get_base_url()))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(Some(id), None)?);

        let res = req
            .send()
            .await
            .map_err(|_| Error::new("Cannot send request to /ClosePayment"))?
            .text()
            .await
            .map_err(|_| Error::new("Cannot convert response from /ClosePayment"))?;

        let res = serde_json::from_str(&res)
            .map_err(|_| Error::new("Cannot parse response from /ClosePayment"))?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all() {
        let invoice_pay = InvoicePay::new(
            "demo".to_string(),
            "1526fec01b5d11f4df4f2160627ce351".to_string(),
        );

        let test_identity = format!(
            "TEST_BLACK_CAT_VPN_{:?}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
        );

        let mut point_of_sale = PointOfSale {
            id: None,
            name: test_identity.clone(),
            address: "test address".to_string(),
            alias: test_identity.clone(),
            mail: "example@gmail.com".to_string(),
            phone: "89211239923".to_string(),
            point_type: PointType::Online,
            website: "http://blackcatvpn.com".to_string(),
        };

        point_of_sale = invoice_pay
            .create_point_of_sale(point_of_sale)
            .await
            .unwrap();

        if point_of_sale.id.is_none() {
            panic!("id is missing, but it is required after creation point of sale");
        }

        point_of_sale = invoice_pay
            .get_point_of_sale(None, Some(test_identity.clone()))
            .await
            .unwrap();

        if point_of_sale.id.is_none() {
            panic!("id is missing, but it is required after getting point of sale by alias");
        }

        point_of_sale = invoice_pay
            .get_point_of_sale(Some(point_of_sale.id.unwrap()), None)
            .await
            .unwrap();

        if point_of_sale.id.is_none() {
            panic!("id is missing, but it is required after getting point of sale by id");
        }

        let mut terminal = Terminal {
            id: None,
            link: None,
            name: test_identity.clone(),
            alias: Some(test_identity.clone()),
            description: "testing...".to_string(),
            terminal_type: TerminalType::Dynamical,
            default_price: 180.0,
        };

        terminal = invoice_pay.create_terminal(terminal).await.unwrap();

        if terminal.id.is_none() {
            panic!("id is missing, but it is required after creation terminal");
        }

        terminal = invoice_pay
            .get_terminal(None, Some(test_identity.clone()))
            .await
            .unwrap();

        if terminal.id.is_none() {
            panic!("id is missing, but it is required after getting terminal by alias");
        }

        terminal = invoice_pay
            .get_terminal(Some(terminal.id.unwrap()), None)
            .await
            .unwrap();

        if terminal.id.is_none() {
            panic!("id is missing, but it is required after getting terminal by id");
        }

        let create_payment_dto = CreatePaymentDto {
            order: Order {
                id: test_identity.clone(),
                currency: Currency::RUB,
                amount: 180.0,
                description: "testing...".to_string(),
            },
            settings: CreatePaymentDtoSettings {
                terminal_id: terminal.id.unwrap(),
                success_url: "https://web.telegram.org/a/#7813325101".to_string(),
                fail_url: "https://web.telegram.org/a/#7813325101".to_string(),
                recur_exp: None,
                recur_freq: None,
            },
            custom_parameters: None,
            receipt: vec![CreatePaymentDtoReceipt {
                name: "Test receipt".to_string(),
                price: 180.0,
                discount: 0.0,
                result_price: 180.0,
                quantity: 1,
            }],
            phone: None,
            mail: None,
            transaction_type: CreatePaymentDtoTransactionType::Once,
        };

        let mut payment_response = invoice_pay
            .create_payment(create_payment_dto)
            .await
            .unwrap();

        payment_response = invoice_pay.get_payment(payment_response.id).await.unwrap();
        
        if payment_response.status != String::from("init") {
            panic!("payment status is invalid: {}", payment_response.status);
        }
        
        payment_response = invoice_pay.cancel_payment(payment_response.id).await.unwrap();

        if payment_response.status != String::from("closed") {
            panic!("payment status is invalid: {}", payment_response.status);
        }
    }
}
