use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use std::collections::HashMap;

mod error;
use crate::error::Error;
pub mod serializable;
use crate::serializable::*;

pub struct InvoicePay {
    api_key: String,
    client: Client,
}

const BASE_URL: &str = "https://api.invoice.su/api/v2";

impl InvoicePay {
    pub fn new(api_key: String) -> InvoicePay {
        InvoicePay {
            api_key,
            client: Client::new(),
        }
    }

    fn serialize_identity_dto(
        &self,
        id: Option<String>,
        alias: Option<String>,
        route: String,
    ) -> Result<String, Error> {
        let mut data: HashMap<&str, String> = HashMap::new();

        if let Some(id) = id {
            data.insert("id", id);
        } else if let Some(alias) = alias {
            data.insert("alias", alias);
        } else {
            return Err(Error::ValidationOneOfTheFieldsError {
                route,
                fields: vec!["id".to_string(), "alias".to_string()],
            });
        }

        Ok(
            serde_json::to_string(&data).map_err(|e| Error::RequestJsonSerializationError {
                route,
                from: "HashMap<&str, String>".to_string(),
                msg: e.to_string(),
            })?,
        )
    }

    // Point of sale

    pub async fn get_point_of_sale(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<PointOfSale, Error> {
        let req = self
            .client
            .post(format!("{}/GetPointOfSale", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(id, alias, "/GetPointOfSale".to_string())?);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/GetPointOfSale".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/GetPointOfSale".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res =
            serde_json::from_str(&res).map_err(|e| Error::ResponseJsonDeserializationError {
                route: "/GetPointOfSale".to_string(),
                to: "PointOfSale".to_string(),
                msg: e.to_string(),
            });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/GetPointOfSale".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }

    pub async fn create_point_of_sale(
        &self,
        point_of_sale: PointOfSale,
    ) -> Result<PointOfSale, Error> {
        let req = self
            .client
            .post(format!("{}/CreatePointOfSale", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .json(&point_of_sale);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/CreatePointOfSale".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/CreatePointOfSale".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res = serde_json::from_str::<PointOfSale>(&res).map_err(|e| {
            Error::ResponseJsonDeserializationError {
                route: "/CreatePointOfSale".to_string(),
                to: "PointOfSale".to_string(),
                msg: e.to_string(),
            }
        });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/CreatePointOfSale".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }

    // Terminal

    pub async fn get_terminal(
        &self,
        id: Option<String>,
        alias: Option<String>,
    ) -> Result<Terminal, Error> {
        let req = self
            .client
            .post(format!("{}/GetTerminal", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(id, alias, "/GetTerminal".to_string())?);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/GetTerminal".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/GetTerminal".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res =
            serde_json::from_str(&res).map_err(|e| Error::ResponseJsonDeserializationError {
                route: "/GetTerminal".to_string(),
                to: "Terminal".to_string(),
                msg: e.to_string(),
            });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/GetTerminal".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }

    pub async fn create_terminal(&self, terminal: Terminal) -> Result<Terminal, Error> {
        let req = self
            .client
            .post(format!("{}/CreateTerminal", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(serde_json::to_string(&terminal).map_err(|e| {
                Error::RequestJsonSerializationError {
                    route: "/CreateTerminal".to_string(),
                    from: "Terminal".to_string(),
                    msg: e.to_string(),
                }
            })?);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/CreateTerminal".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/CreateTerminal".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res =
            serde_json::from_str(&res).map_err(|e| Error::ResponseJsonDeserializationError {
                route: "/CreateTerminal".to_string(),
                to: "Terminal".to_string(),
                msg: e.to_string(),
            });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/CreateTerminal".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }

    // Payment

    pub async fn get_payment(&self, id: String) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/GetPayment", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(Some(id), None, "/GetPayment".to_string())?);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/GetPayment".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/GetPayment".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res =
            serde_json::from_str(&res).map_err(|e| Error::ResponseJsonDeserializationError {
                route: "/GetPayment".to_string(),
                to: "PaymentResponseDto".to_string(),
                msg: e.to_string(),
            });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/GetPayment".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }

    pub async fn create_payment(
        &self,
        payment_dto: CreatePaymentDto,
    ) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/CreatePayment", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(serde_json::to_string(&payment_dto).map_err(|e| {
                Error::RequestJsonSerializationError {
                    route: "/CreatePayment".to_string(),
                    from: "CreatePaymentDto".to_string(),
                    msg: e.to_string(),
                }
            })?);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/CreatePayment".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/CreatePayment".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res =
            serde_json::from_str(&res).map_err(|e| Error::ResponseJsonDeserializationError {
                route: "/CreatePayment".to_string(),
                to: "PaymentResponseDto".to_string(),
                msg: e.to_string(),
            });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/CreatePayment".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }

    pub async fn cancel_payment(&self, id: String) -> Result<PaymentResponseDto, Error> {
        let req = self
            .client
            .post(format!("{}/ClosePayment", BASE_URL))
            .header(AUTHORIZATION, format!("Basic {}", self.api_key))
            .body(self.serialize_identity_dto(Some(id), None, "/ClosePayment".to_string())?);

        let res = req
            .send()
            .await
            .map_err(|e| Error::RequestSendingError {
                route: "/ClosePayment".to_string(),
                msg: e.to_string(),
            })?
            .text()
            .await
            .map_err(|e| Error::ConversationError {
                route: "/ClosePayment".to_string(),
                from: "Response<&str>".to_string(),
                to: "UTF-8 &str".to_string(),
                msg: e.to_string(),
            })?;

        let deserialized_res =
            serde_json::from_str(&res).map_err(|e| Error::ResponseJsonDeserializationError {
                route: "/ClosePayment".to_string(),
                to: "PaymentResponseDto".to_string(),
                msg: e.to_string(),
            });

        if deserialized_res.is_err() {
            return match serde_json::from_str::<ApiErrorResponse>(&res) {
                Ok(api_err) => Err(Error::ApiError {
                    route: "/ClosePayment".to_string(),
                    code: api_err.error,
                    msg: api_err.description,
                    additions: api_err.additions,
                }),
                Err(_) => deserialized_res,
            };
        }

        Ok(deserialized_res?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all() {
        let invoice_pay =
            InvoicePay::new("ZGVtbzoxNTI2ZmVjMDFiNWQxMWY0ZGY0ZjIxNjA2MjdjZTM1MQ==".to_string());

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
            website: "https://blackcatvpn.com".to_string(),
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

        payment_response = invoice_pay
            .cancel_payment(payment_response.id)
            .await
            .unwrap();

        if payment_response.status != String::from("closed") {
            panic!("payment status is invalid: {}", payment_response.status);
        }
    }
}
