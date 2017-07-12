extern crate chrono;
extern crate serde_json;
extern crate xero;

use chrono::NaiveDate;
use serde_json as json;
use xero::accounting::*;
use xero::encoding::XmlSerializable;

fn _xml(example: &'static str) -> Option<String> {
    Some(String::from(&example[1..]))
}

fn _json(example: &'static str) -> Option<String> {
    Some(String::from(example))
}

#[test]
fn serialize_invoice_params() {
    let invoice = InvoiceParams{
        invoice_type: InvoiceType::AccountsReceivable,
        contact: ContactIdParams{id: "eaa28f49-6028-4b6e-bb12-d8f6278073fc"},
        date: None,
        date_due: None,
        status: None,
        url: None,
        sent_to_contact: None,
        line_amount_types: None,
        line_items: vec![],
    };

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
        contact: ContactIdParams{id: "eaa28f49-6028-4b6e-bb12-d8f6278073fc"},
        date: Some(NaiveDate::from_ymd(2009, 08, 30)),
        date_due: Some(NaiveDate::from_ymd(2009, 09, 20)),
        url: Some("https://twitter.com/SuperTransparentInvoices/status/865425833631993856"),
        status: Some(InvoiceStatus::Authorised),
        sent_to_contact: Some(true),
        line_amount_types: Some(LineAmountType::Exclusive),
        line_items: vec![
            LineItemParams{
                item_code: None,
                description: "Consulting services as agreed",
                quantity: 5.0,
                unit_amount: 120.0,
                account_code: "200",
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
<DateDue>2009-09-20</DateDue>
<Url>https://twitter.com/SuperTransparentInvoices/status/865425833631993856</Url>
<Status>AUTHORISED</Status>
<SentToContact>true</SentToContact>
<LineItems>
  <LineItem>
    <Description>Consulting services as agreed</Description>
    <Quantity>5</Quantity>
    <UnitAmount>120</UnitAmount>
    <AccountCode>200</AccountCode>
  </LineItem>
</LineItems>"));
}
