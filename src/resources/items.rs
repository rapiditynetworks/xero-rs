use client::Client;
use encoding::{XmlError, XmlSerializable, XmlWriter};
use error::Error;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDetails {
    pub unit_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "COGSAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
}

impl<'a> XmlSerializable for ItemDetails {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("UnitPrice", &self.unit_price)?;
        xml.element_opt("AccountCode", &self.account_code)?;
        xml.element_opt("COGSAccountCode", &self.cogs_account_code)?;
        xml.element_opt("TaxType", &self.tax_type)
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemParams<'a> {
    pub code: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_asset_account_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchased: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<ItemDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_details: Option<ItemDetails>,
}

impl<'a> XmlSerializable for ItemParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("Code", &self.code)?;
        xml.element_opt("Description", &self.description)?;
        xml.element_opt("PurchaseDescription", &self.purchase_description)?;
        xml.element_opt("PurchaseDetails", &self.purchase_details)?;
        xml.element_opt("SalesDetails", &self.purchase_details)?;
        xml.element_opt("Name", &self.name)?;
        xml.element_opt("InventoryAssetAccountCode", &self.inventory_asset_account_code)?;
        xml.element_opt("IsSold", &self.is_sold)?;
        xml.element_opt("IsPurchased", &self.is_purchased)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    /* TODO: GET */
}

impl Item {
    pub fn put(client: &Client, item: ItemParams) -> Result<Item, Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.element("Item", &item)?;
        }
        let items: Items = client.put("/Items", body.as_slice())?;
        Ok(items.items.into_iter().next().expect("Expected item after successful PUT"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Items {
    pub items: Vec<Item>,
}

impl Items {
    pub fn put(client: &Client, items: Vec<ItemParams>) -> Result<Items, Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.array("Items", "Item", &items)?;
        }
        client.put("/Items", body.as_slice())
    }
}
