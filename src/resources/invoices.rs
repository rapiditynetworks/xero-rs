use chrono::NaiveDate;
use client::Client;
use encoding::{XmlError, XmlSerializable, XmlWriter};
use error::Error;

use resources::contacts::ContactIdParams;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum InvoiceType {
    #[serde(rename = "ACCPAY")]
    AccountsPayable,
    #[serde(rename = "ACCREC")]
    AccountsReceivable,
}

impl XmlSerializable for InvoiceType {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            InvoiceType::AccountsPayable => xml.write("ACCPAY"),
            InvoiceType::AccountsReceivable => xml.write("ACCREC"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    Draft,
    Submitted,
    Deleted,
    Authorised, // NOTE: International spelling
    Paid,
    Voided,
}

impl XmlSerializable for InvoiceStatus {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            InvoiceStatus::Draft => xml.write("DRAFT"),
            InvoiceStatus::Submitted => xml.write("SUBMITTED"),
            InvoiceStatus::Deleted => xml.write("DELETED"),
            InvoiceStatus::Authorised => xml.write("AUTHORISED"),
            InvoiceStatus::Paid => xml.write("PAID"),
            InvoiceStatus::Voided => xml.write("VOIDED"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")] // NOTE: Not SCREAMING like other enums
pub enum LineAmountType {
    Exclusive,
    Inclusive,
    NoTax,
}

impl XmlSerializable for LineAmountType {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            LineAmountType::Exclusive => xml.write("Exclusive"),
            LineAmountType::Inclusive => xml.write("Inclusive"),
            LineAmountType::NoTax => xml.write("NoTax"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_code: Option<&'a str>,
    pub description: &'a str,
    pub quantity: f64,
    pub unit_amount: f64,
    pub account_code: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_rate: Option<u32>,
}

impl<'a> XmlSerializable for LineItemParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        if let Some(code) = self.item_code {
            xml.element("UnitAmount", &code)?;
        }
        xml.element("Description", &self.description)?;
        xml.element("Quantity", &self.quantity)?;
        xml.element("UnitAmount", &self.unit_amount)?;
        xml.element("AccountCode", &self.account_code)?;
        if let Some(rate) = self.discount_rate {
            xml.element("DiscountRate", &rate)?;
        }
        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceParams<'a> {
    #[serde(rename = "Type")]
    pub invoice_type: InvoiceType,
    pub contact: ContactIdParams<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_due: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    pub line_items: Vec<LineItemParams<'a>>,
}

impl<'a> XmlSerializable for InvoiceParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("Type", &self.invoice_type)?;
        xml.element("Contact", &self.contact)?;
        if let Some(date) = self.date { xml.element("Date", &date.format("%Y-%m-%d").to_string())?; }
        if let Some(date_due) = self.date_due { xml.element("DateDue", &date_due.format("%Y-%m-%d").to_string())?; }
        if let Some(url) = self.url { xml.element("Url", &url)?; }
        if let Some(status) = self.status { xml.element("Status", &status)?; }
        if let Some(sent_to_contact) = self.sent_to_contact { xml.element("SentToContact", &sent_to_contact)?; }
        xml.array("LineItems", "LineItem", &self.line_items)
    }
}

pub struct Invoice {
    /* TODO: Retrieve */
}

impl Invoice {
    pub fn create(client: &Client, invoice: InvoiceParams) -> Result<(), Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.element("Invoice", &invoice)?;
        }
        client.put("/Invoices", body.as_slice())?;
        Ok(())
    }

    pub fn create_many(client: &Client, invoices: Vec<InvoiceParams>) -> Result<(), Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.array("Invoices", "Invoice", &invoices)?;
        }
        client.put("/Invoices", body.as_slice())?;
        Ok(())
    }
}
