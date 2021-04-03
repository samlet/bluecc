use chrono::{DateTime, Utc};
use common::prelude::*;
use serde_json::json;
// use crate::resources::common_date_format;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct OrderHeader{
    #[serde(flatten)]
    pub id: OrderHeaderId,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_channel_enum_id: Option<String>,
    // #[serde(with = "common_date_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    // #[serde(with = "common_date_format")]
     #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_date: Option<DateTime<Utc>>,
    // #[serde(with = "common_date_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pick_sheet_printed_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_attempt_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_uom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_facility_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_site_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_order_shopping_list_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_inventory_issuance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rush_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_sub_total: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grand_total: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_viewed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_per_shipment: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct OrderHeaderId {
    pub order_id: Option<String>,
}

impl Object for OrderHeader{
    type Id = OrderHeaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order_header"
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::delegators::{pretty, Delegator};
    use serde_json::json;
    use std::collections::HashMap;

    fn print_errs(errors:&crate::Error){
        eprintln!("Error level - description");
        errors
            .iter()
            .enumerate()
            .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

        if let Some(backtrace) = errors.backtrace() {
            eprintln!("{:?}", backtrace);
        }
    }

    #[tokio::test]
    async fn serialize_obj_works() -> crate::Result<()> {
        std::env::set_var("RUST_LOG", "info,deles=debug");
        env_logger::init();

        let order_id = "DEMO10090";

        let p: OrderHeader = serde_json::from_value(json!({
          "orderTypeId": "SALES_ORDER",
          "grandTotal": 50.85,
          "salesChannelEnumId": "WEB_SALES_CHANNEL",
          "statusId": "ORDER_APPROVED",
          "remainingSubTotal": 38.40,
          "orderId": order_id,
          "priority": "2",
          "createdBy": "admin",
          "currencyUom": "USD",
          "productStoreId": "9000",
          "orderDate": utc_fmt("2008-04-23 16:49:27.392")?,
          "visitId": "10002",
          "entryDate": utc_fmt("2008-04-23 16:49:27.392")?,
          "invoicePerShipment": "Y",
          "webSiteId": "WebStore"
        }))?;

        println!("{}", pretty(&p));
        // let vals=serde_json::to_value(&p)?;
        let json_str = serde_json::to_string(&p)?;
        println!("{}", json_str);

        // let values:HashMap<String,String> = serde_json::from_str(json_str.as_str())?;
        // let delegator = Delegator::new().await?;

        // if let Some(values)=vals.as_object(){
        // println!("{:?}", values);
        // let changes = delegator.store_string_map("OrderHeader", values).await;
        // if let Err(ref errors) = changes {
        //     print_errs(errors);
        // }
        // println!("changes: {}", changes);
        // }

        Ok(())
    }

    #[tokio::test]
    async fn store_works() -> crate::Result<()> {
        std::env::set_var("RUST_LOG", "info,deles=debug");
        env_logger::init();

        let order_id = "bluecc_DEMO10090";

        // as a string-map
        let p: HashMap<String, String> = serde_json::from_value(json!({
          "orderTypeId": "SALES_ORDER",
          "grandTotal": "50.85",
          "salesChannelEnumId": "WEB_SALES_CHANNEL",
          "statusId": "ORDER_APPROVED",
          "remainingSubTotal": "38.40",
          "orderId": order_id,
          "priority": "2",
          "createdBy": "admin",
          "currencyUom": "USD",
          "productStoreId": "9000",
          "orderDate": "2008-04-23 16:49:27.392",
          "visitId": "10002",
          "entryDate": "2008-04-23 16:49:27.392",
          "invoicePerShipment": "Y",
          "webSiteId": "WebStore"
        }))?;

        let delegator = Delegator::new().await?;

        // if let Some(values)=p.as_object(){
        let changes = delegator.store_string_map("OrderHeader", p).await;
        if let Err(ref errors) = changes {
            print_errs(errors);
        }
        println!("changes: {:?}", changes);
        // }

        Ok(())
    }
}
