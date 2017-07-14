use client::Client;
use encoding::{XmlError, XmlSerializable, XmlWriter};
use error::Error;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContactStatus {
    Active,
    Archived,
}

impl XmlSerializable for ContactStatus {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            ContactStatus::Active => xml.write("ACTIVE"),
            ContactStatus::Archived => xml.write("ARCHIVED"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressType {
    #[serde(rename = "POBOX")]
    POBox,
    Street,
    Delivery,
}

impl XmlSerializable for AddressType {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            AddressType::POBox => xml.write("POBOX"),
            AddressType::Street => xml.write("STREET"),
            AddressType::Delivery => xml.write("DELIVERY"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    pub address_type: AddressType,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub address_line1: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub address_line2: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub address_line3: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub address_line4: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub city: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub region: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub postal_code: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub country: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub attention_to: String,
}

impl<'a> XmlSerializable for Address {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("AddressType", &self.address_type)?;
        if !self.address_line1.is_empty() { xml.element("AddressLine1", &self.address_line1)?; }
        if !self.address_line2.is_empty() { xml.element("AddressLine2", &self.address_line2)?; }
        if !self.address_line3.is_empty() { xml.element("AddressLine3", &self.address_line3)?; }
        if !self.address_line4.is_empty() { xml.element("AddressLine4", &self.address_line4)?; }
        if !self.city.is_empty() { xml.element("City", &self.city)?; }
        if !self.region.is_empty() { xml.element("Region", &self.region)?; }
        if !self.postal_code.is_empty() { xml.element("PostalCode", &self.postal_code)?; }
        if !self.country.is_empty() { xml.element("Country", &self.country)?; }
        if !self.attention_to.is_empty() { xml.element("AttentionTo", &self.attention_to)?; }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhoneType {
    Default,
    #[serde(rename = "DDI")]
    DirectDial,
    Mobile,
    Fax,
}

impl XmlSerializable for PhoneType {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        match *self {
            PhoneType::Default => xml.write("DEFAULT"),
            PhoneType::DirectDial => xml.write("DDI"),
            PhoneType::Mobile => xml.write("MOBILE"),
            PhoneType::Fax => xml.write("FAX"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Phone {
    pub phone_type: PhoneType,
    pub phone_number: String,
    pub phone_area_code: String,
    pub phone_country_code: String,
}

impl<'a> XmlSerializable for Phone {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("PhoneType", &self.phone_type)?;
        xml.element("PhoneNumber", &self.phone_number)?;
        xml.element("PhoneAreaCode", &self.phone_area_code)?;
        xml.element("PhoneCountryCode", &self.phone_country_code)
    }
}

#[derive(Default, Serialize)]
pub struct ContactIdParams<'a> {
    #[serde(rename = "ContactID")]
    pub contact_id: &'a str, // Required
}

impl<'a> XmlSerializable for ContactIdParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("ContactID", &self.contact_id)
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContactParams<'a> {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<ContactStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skype_user_name: Option<&'a str>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub contact_persons: Option<Vec<ContactPerson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_receivable_tax_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_payable_tax_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customer: Option<bool>,
    // pub default_currency: Option<_>,
    // pub xero_network_key: Option<_>,
    // ...
}

impl<'a> XmlSerializable for ContactParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element_opt("ContactID", &self.contact_id)?;
        xml.element_opt("ContactNumber", &self.contact_number)?;
        xml.element_opt("AccountNumber", &self.account_number)?;
        xml.element_opt("ContactStatus", &self.contact_status)?;
        xml.element_opt("Name", &self.name)?;
        xml.element_opt("FirstName", &self.first_name)?;
        xml.element_opt("LastName", &self.last_name)?;
        xml.element_opt("EmailAddress", &self.email_address)?;
        xml.element_opt("SkypeUserName", &self.skype_user_name)?;
        xml.element_opt("BankAccountDetails", &self.bank_account_details)?;
        xml.element_opt("TaxNumber", &self.tax_number)?;
        xml.element_opt("AccountsReceivableTaxType", &self.accounts_receivable_tax_type)?;
        xml.element_opt("AccountsPayableTaxType", &self.accounts_payable_tax_type)?;
        if let Some(ref addresses) = self.addresses {
            xml.array("Addresses", "Address", &addresses)?;
        }
        if let Some(ref phones) = self.phones {
            xml.array("Phones", "Phone", &phones)?;
        }
        xml.element_opt("IsSupplier", &self.is_supplier)?;
        xml.element_opt("IsCustomer", &self.is_customer)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contact {
    #[serde(rename = "ContactID")]
    pub contact_id: String,
    pub contact_status: ContactStatus,
    pub name: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub email_address: String,
    #[serde(default)]
    pub skype_user_name: String,
    #[serde(default)]
    pub bank_account_details: String,
    #[serde(default)]
    pub tax_number: String,
    pub accounts_receivable_tax_type: Option<String>,
    pub accounts_payable_tax_type: Option<String>,
    pub addresses: Vec<Address>,
    pub phones: Vec<Phone>,
    pub is_supplier: bool,
    pub is_customer: bool,
    // pub default_currency
    // pub updated_date_utc
    // ...
}

impl Contact {
    pub fn put(client: &Client, params: ContactParams) -> Result<Contact, Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.element("Contact", &params)?;
        }
        let contacts: Contacts = client.put("/Contacts", body.as_slice())?;
        Ok(contacts.contacts.into_iter().next().expect("Expected contact after successful PUT"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contacts {
    pub contacts: Vec<Contact>,
}

impl Contacts {
    pub fn get(client: &Client) -> Result<Contacts, Error> {
        client.get("/Contacts")
    }

    pub fn put(client: &Client, params: Vec<ContactParams>) -> Result<Contacts, Error> {
        let mut body = Vec::new();
        {
            let mut xml = XmlWriter::new(&mut body);
            xml.array("Contacts", "Contact", &params)?;
        }
        client.put("/Contacts", body.as_slice())
    }
}
