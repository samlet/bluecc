use bigdecimal::BigDecimal;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentApplicationCr {
    pub payment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_auth_geo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_applied: Option<BigDecimal>,
}

impl CreatePaymentApplicationCr {
    pub fn new(payment_id: String) -> Self {
        CreatePaymentApplicationCr {
            payment_id,
            to_payment_id: Default::default(),
            invoice_id: Default::default(),
            billing_account_id: Default::default(),
            tax_auth_geo_id: Default::default(),
            amount_applied: Default::default(),
        }
    }
}

const CREATE_PAYMENT_APPLICATION: &'static str = "createPaymentApplication";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct CreatePaymentApplicationResp {
    pub amount_applied: Option<BigDecimal>,
    pub payment_application_id: String,
    pub payment_type_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProposalData{
    pub data_type: String,
    pub data: serde_json::Value,
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn from_json_works() -> anyhow::Result<()> {
        let cr:CreatePaymentApplicationCr=serde_json::from_value(json!({
                    "invoiceId": "demo11001",
                    "amountApplied": 543.23,
                    "paymentApplicationId": "demo11000",
                    "paymentId": "demo10001"
                }))?;
        println!("{} -> {}", cr.payment_id, cr.invoice_id.unwrap());
        Ok(())
    }

    #[test]
    fn propsal_works() -> anyhow::Result<()> {
        let ppd:ProposalData=serde_json::from_value(json!({
                    "dataType": "createPaymentApplication",
                    "data": {
                      "invoiceId": "demo11001",
                      "amountApplied": 543.23,
                      "paymentApplicationId": "demo11000",
                      "paymentId": "demo10001"
                    }
                }))?;
        println!("{} ->", ppd.data_type);
        match ppd.data_type.as_str(){
            CREATE_PAYMENT_APPLICATION => {
                let cr:CreatePaymentApplicationCr=serde_json::from_value(ppd.data)?;
                println!("{} -> {}", cr.payment_id, cr.invoice_id.unwrap());
            }
            _ => {
                assert!(false);
            }
        }

        Ok(())
    }
}

