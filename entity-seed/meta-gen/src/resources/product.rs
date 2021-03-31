use chrono::{DateTime, Utc};
use crate::params::Object;
use crate::{SrvDeles, SrvResp, GenericError, DynamicValue};
use std::collections::HashMap;
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Product{
    #[serde(flatten)]
    pub id: ProductId,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_product_category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facility_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_discontinuation_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_discontinuation_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_disc_when_not_avail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_detail_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_screen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_item_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_inventory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_included: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces_included: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_uom_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_weight: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_weight: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_height: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_height: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_width: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_width: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_depth: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_depth: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diameter_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_diameter: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rating: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_type_enum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_shipping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_promotions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_variant_method_enum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_geo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_method_enum_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_of_material_level: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserv_max_persons: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserv_2nd_pp_perc: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserv_nth_pp_perc: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_user_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_shipping_box: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipment_box_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_id_filled_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_decimal_quantity: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProductId {
    pub product_id: Option<String>,
}

impl Object for Product{
    type Id = ProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "product"
    }
}

/// The parameters for `Product::create`.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProduct<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<&'a str>,
}

impl<'a> CreateProduct<'a> {
    pub fn new() -> Self {
        CreateProduct{
            product_id: Default::default(),
        }
    }
}

const CREATE_PRODUCT: &'static str = "createProduct";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct CreateProductResp {
    pub product_id: Option<String>,
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use deles::delegators::pretty;

    #[test]
    fn xml_works() -> anyhow::Result<()> {
        let raw=r#"<Product productId="WG-9943" productTypeId="FINISHED_GOOD"/>"#;
        let prod:Product=serde_xml_rs::from_str(raw)?;
        println!("{}", pretty(&prod));
        Ok(())
    }
}

