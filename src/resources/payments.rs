use chrono::{NaiveDate, NaiveDateTime, UTC};
use client::Client;
use encoding::{XmlError, XmlSerializable, XmlWriter};
use error::Error;

use resources::invoices::InvoiceSummary;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum PaymentStatus {
    #[serde(rename = "AUTHORISED")]
    Authorised,
    #[serde(rename = "DELETED")]
    Deleted,
}

impl XmlSerializable for PaymentStatus {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentStatus::Authorised => xml.write("AUTHORISED"),
            PaymentStatus::Deleted => xml.write("DELETED"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum PaymentType {
    #[serde(rename = "ACCRECPAYMENT")]
    AccountsReceivable,
    #[serde(rename = "ACCPAYPAYMENT")]
    AccountsPayable,
    #[serde(rename = "ARCREDITPAYMENT")]
    ARCredit,
    #[serde(rename = "APCREDITPAYMENT")]
    APCredit,

    /// Refunds
    #[serde(rename = "APOVERPAYMENTPAYMENT")]
    AROverpayment,
    #[serde(rename = "AROVERPAYMENTPAYMENT")]
    APOverpayment,
    #[serde(rename = "ARPREPAYMENTPAYMENT")]
    ARPrepayment,
    #[serde(rename = "APPREPAYMENTPAYMENT")]
    APPrepayment,
}

impl XmlSerializable for PaymentType {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentType::AccountsReceivable => xml.write("ACCRECPAYMENT"),
            PaymentType::AccountsPayable => xml.write("ACCPAYPAYMENT"),
            PaymentType::ARCredit => xml.write("ARCREDITPAYMENT"),
            PaymentType::APCredit => xml.write("APCREDITPAYMENT"),
            PaymentType::AROverpayment => xml.write("AROVERPAYMENTPAYMENT"),
            PaymentType::APOverpayment => xml.write("APOVERPAYMENTPAYMENT"),
            PaymentType::ARPrepayment => xml.write("ARPREPAYMENTPAYMENT"),
            PaymentType::APPrepayment => xml.write("APPREPAYMENTPAYMENT"),
        }
    }
}

#[derive(Serialize)]
pub enum PaymentInvoice<'a> {
    #[serde(rename = "InvoiceID")]
    InvoiceId(&'a str),
    InvoiceNumber(&'a str),
}

impl<'a> XmlSerializable for PaymentInvoice<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentInvoice::InvoiceId(id) => xml.element("InvoiceID", &id),
            PaymentInvoice::InvoiceNumber(number) => xml.element("InvoiceNumber", &number),
        }
    }
}

#[derive(Serialize)]
pub enum PaymentCreditNote<'a> {
    #[serde(rename = "CreditNoteID")]
    CreditNoteId(&'a str),
    CreditNoteNumber(&'a str),
}

impl<'a> XmlSerializable for PaymentCreditNote<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentCreditNote::CreditNoteId(id) => xml.element("CreditNoteID", &id),
            PaymentCreditNote::CreditNoteNumber(number) => xml.element("CreditNumber", &number),
        }
    }
}

#[derive(Serialize)]
pub enum PaymentPrepayment<'a> {
    #[serde(rename = "CreditNoteID")]
    PrepaymentId(&'a str),
}

impl<'a> XmlSerializable for PaymentPrepayment<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentPrepayment::PrepaymentId(id) => xml.element("PrepaymentID", &id),
        }
    }
}

#[derive(Serialize)]
pub enum PaymentOverpayment<'a> {
    #[serde(rename = "OverpaymentID")]
    OverpaymentId(&'a str),
}

impl<'a> XmlSerializable for PaymentOverpayment<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentOverpayment::OverpaymentId(id) => xml.element("OverpaymentID", &id),
        }
    }
}

#[derive(Serialize)]
pub enum PaymentAccount<'a> {
    #[serde(rename = "AccountID")]
    AccountId(&'a str),
    Code(&'a str),
}

impl<'a> XmlSerializable for PaymentAccount<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PaymentAccount::AccountId(id) => xml.element("AccountID", &id),
            PaymentAccount::Code(id) => xml.element("Code", &id),
        }
    }
}

/// ... Some fields missing ...
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<PaymentInvoice<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<PaymentCreditNote<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepayment: Option<PaymentPrepayment<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overpayment: Option<PaymentOverpayment<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<PaymentAccount<'a>>,

    pub date: NaiveDate,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>, // ie. a memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reconciled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    // ...
}

impl<'a> Default for PaymentParams<'a> {
    fn default() -> Self {
        PaymentParams{
            invoice: None,
            credit_note: None,
            prepayment: None,
            overpayment: None,
            account: None,
            date: UTC::today().naive_utc(),
            amount: 0.0,
            reference: None,
            is_reconciled: None,
            status: None,
            payment_type: None,
        }
    }
}

impl<'a> XmlSerializable for PaymentParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element_opt("Invoice", &self.invoice)?;
        xml.element_opt("CreditNote", &self.credit_note)?;
        xml.element_opt("Prepayment", &self.prepayment)?;
        xml.element_opt("Overpayment", &self.overpayment)?;
        xml.element_opt("Account", &self.account)?;
        xml.element("Date", &self.date.format("%Y-%m-%d").to_string())?;
        xml.element("Amount", &self.amount)?;
        xml.element_opt("Reference", &self.reference)?;
        xml.element_opt("IsReconciled", &self.is_reconciled)?;
        xml.element_opt("Status", &self.status)?;
        xml.element_opt("PaymentType", &self.payment_type)
    }
}

/// ... Some fields missing ...
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    #[serde(rename = "PaymentID")]
    pub payment_id: String,
    // FIXME: Invoice has DateString, but I guess Payment might not---
    // TODO: see if this is actually Optional or whether we should just use C# Date format
    // #[serde(rename = "DateString")]
    // pub date: NaiveDateTime,
    pub amount: f64,
    pub payment_type: PaymentType,
    pub status: PaymentStatus,
    pub is_reconciled: bool,
    // account: Option<_>,
    pub invoice: Option<InvoiceSummary>,
    // ...
}

impl Payment {
    pub fn put(client: &Client, payment: PaymentParams) -> Result<Payment, Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.element("Payment", &payment)?;
        }
        let payments: Payments = client.put("/Payments", body.as_slice())?;
        Ok(payments.payments.into_iter().next().expect("Expected payment after successful PUT"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payments {
    pub payments: Vec<Payment>,
}

impl Payments {
    pub fn put(client: &Client, payments: Vec<PaymentParams>) -> Result<Payments, Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.array("Payments", "Payment", &payments)?;
        }
        client.put("/Payments", body.as_slice())
    }
}
