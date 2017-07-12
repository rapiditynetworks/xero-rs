use encoding::{XmlError, XmlSerializable, XmlWriter};

#[derive(Serialize)]
pub struct ContactIdParams<'a> {
    #[serde(rename = "ContactID")]
    pub id: &'a str,
}

impl<'a> XmlSerializable for ContactIdParams<'a> {
    fn write(&self, xml: &mut XmlWriter) ->  Result<(), XmlError> {
        xml.element("ContactID", &self.id)
    }
}
