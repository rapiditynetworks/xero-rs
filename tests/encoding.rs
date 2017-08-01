extern crate bigdecimal;
extern crate chrono;
extern crate serde_json;
extern crate xero;

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde_json as json;
use xero::accounting::*;
use xero::encoding::XmlSerializable;

fn _xml(example: &'static str) -> Option<String> {
    if example.chars().next() == Some('\n') {
        Some(String::from(&example[1..]))
    } else {
        Some(String::from(example))
    }
}

fn _json(example: &'static str) -> Option<String> {
    Some(String::from(example))
}

#[test]
fn serialize_item_details() {
    let item = ItemDetails{
        unit_price: Some(BigDecimal::from(0).with_scale(4)),
        account_code: None,
        cogs_account_code: None,
        tax_type: None
    };

    assert_eq!(item.to_xml().ok(), _xml("<UnitPrice>0.0000</UnitPrice>"));
    assert_eq!(json::to_string(&item).ok(), _json(r#"{"UnitPrice":"0.0000"}"#));
}

#[test]
fn serialized_payment_params() {
    let payment_params = PaymentParams{
        invoice: None,
        credit_note: None,
        prepayment: None,
        overpayment: None,
        account: None,
        date: NaiveDate::from_ymd(2009, 08, 30),
        amount: BigDecimal::from(0).with_scale(4),
        reference: None,
        is_reconciled: None,
        status: None,
        payment_type: None
    };

    assert_eq!(payment_params.to_xml().ok(), _xml("
<Date>2009-08-30</Date>
<Amount>0.0000</Amount>"));
    assert_eq!(json::to_string_pretty(&payment_params).ok(), _json(r#"{
  "Date": "2009-08-30",
  "Amount": "0.0000"
}"#));
}

#[test]
fn dserialized_payment() {
    let data = r#"{
        "PaymentID": "payment-id",
        "Amount": 0.0000,
        "PaymentType": "ACCRECPAYMENT",
        "Status": "AUTHORISED",
        "IsReconciled": true
    }"#;

    let payment: Payment = json::from_str(&data).unwrap();

    assert_eq!(payment, Payment{
        payment_id: String::from("payment-id"),
        amount: BigDecimal::from(0).with_scale(4),
        payment_type: PaymentType::AccountsReceivable,
        status: PaymentStatus::Authorised,
        is_reconciled: true,
        invoice: None
    });
}

#[test]
fn serialize_invoice_params() {
    let mut invoice = InvoiceParams::default();
    invoice.contact = ContactIdParams{contact_id: "eaa28f49-6028-4b6e-bb12-d8f6278073fc"};

    assert_eq!(invoice.to_xml().ok(), _xml("
<Type>ACCREC</Type>
<Contact>
  <ContactID>eaa28f49-6028-4b6e-bb12-d8f6278073fc</ContactID>
</Contact>
<LineItems />"));
    assert_eq!(json::to_string_pretty(&invoice).ok(), _json(r#"{
  "Type": "ACCREC",
  "Contact": {
    "ContactID": "eaa28f49-6028-4b6e-bb12-d8f6278073fc"
  },
  "LineItems": []
}"#));

    let invoice = InvoiceParams{
        invoice_type: InvoiceType::AccountsReceivable,
        contact: ContactIdParams{contact_id: "eaa28f49-6028-4b6e-bb12-d8f6278073fc"},
        date: Some(NaiveDate::from_ymd(2009, 08, 30)),
        due_date: Some(NaiveDate::from_ymd(2009, 09, 20)),
        invoice_number: Some("0010"),
        reference: Some("Ref:ABC"),
        url: Some("https://twitter.com/SuperTransparentInvoices/status/865425833631993856"),
        status: Some(InvoiceStatus::Authorised),
        sent_to_contact: Some(true),
        line_amount_types: Some(LineAmountType::Exclusive),
        line_items: vec![
            LineItemParams{
                item_code: None,
                description: "Consulting services as agreed",
                quantity: Some(5.0),
                unit_amount: Some(BigDecimal::from(0).with_scale(4)),
                line_amount: Some(BigDecimal::from(0).with_scale(4)),
                tax_amount: None,
                account_code: Some("200"),
                discount_rate: None,
            }
        ],
    };

    assert_eq!(invoice.to_xml().ok(), _xml("
<Type>ACCREC</Type>
<Contact>
  <ContactID>eaa28f49-6028-4b6e-bb12-d8f6278073fc</ContactID>
</Contact>
<Date>2009-08-30</Date>
<DueDate>2009-09-20</DueDate>
<InvoiceNumber>0010</InvoiceNumber>
<Reference>Ref:ABC</Reference>
<Url>https://twitter.com/SuperTransparentInvoices/status/865425833631993856</Url>
<Status>AUTHORISED</Status>
<SentToContact>true</SentToContact>
<LineAmountTypes>Exclusive</LineAmountTypes>
<LineItems>
  <LineItem>
    <Description>Consulting services as agreed</Description>
    <Quantity>5</Quantity>
    <UnitAmount>0.0000</UnitAmount>
    <LineAmount>0.0000</LineAmount>
    <AccountCode>200</AccountCode>
  </LineItem>
</LineItems>"));
}
