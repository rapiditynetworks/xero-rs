use bigdecimal::BigDecimal;
use xml::EventWriter;
use xml::writer::{EmitterConfig, XmlEvent};

pub use xml::writer::Error as XmlError;

pub struct XmlWriter<'a> {
    w: EventWriter<&'a mut Vec<u8>>,
}

impl<'a> XmlWriter<'a> {
    pub fn new(body: &'a mut Vec<u8>) -> XmlWriter<'a> {
        let writer = EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(false)
            .create_writer(body);
        XmlWriter{w: writer}
    }

    pub fn write(&mut self, text: &str) -> Result<(), XmlError> {
        self.w.write(text)
    }

    pub fn start_element(&mut self, element: &str) -> Result<(), XmlError> {
        self.w.write(XmlEvent::start_element(element))
    }

    pub fn end_element(&mut self) -> Result<(), XmlError> {
        self.w.write(XmlEvent::end_element())
    }

    pub fn element<T: XmlSerializable>(&mut self, label: &str, content: &T) -> Result<(), XmlError> {
        self.start_element(label)?;
        content.write(self)?;
        self.end_element()
    }

    pub fn element_opt<T: XmlSerializable>(&mut self, label: &str, content: &Option<T>) -> Result<(), XmlError> {
        if let Some(ref content) = *content {
            self.start_element(label)?;
            content.write(self)?;
            self.end_element()
        } else {
            Ok(())
        }
    }

    pub fn array<T: XmlSerializable>(&mut self, array: &str, element: &str, items: &Vec<T>) -> Result<(), XmlError> {
        self.start_element(array)?;
        for item in items {
            self.element(element, item)?;
        }
        self.end_element()
    }
}

pub trait XmlSerializable {
    fn write(&self, &mut XmlWriter) -> Result<(), XmlError>;

    fn to_xml(&self) -> Result<String, XmlError> {
        let mut body = Vec::new();
        {
            let mut writer = XmlWriter::new(&mut body);
            self.write(&mut writer)?;
        }
        Ok(String::from_utf8(body).unwrap()) // FIXME: Don't unwrap? or document panic
    }
}

impl XmlSerializable for bool {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(match *self {true => "true", false => "false"})
    }
}
impl XmlSerializable for i32 {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.to_string())
    }
}
impl XmlSerializable for i64 {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.to_string())
    }
}
impl XmlSerializable for u32 {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.to_string())
    }
}
impl XmlSerializable for u64 {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.to_string())
    }
}
impl XmlSerializable for f32 {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.to_string())
    }
}
impl XmlSerializable for f64 {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.to_string())
     }
}
impl<'a> XmlSerializable for &'a str {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(self)
    }
}
impl XmlSerializable for String {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(self)
    }
}

impl XmlSerializable for BigDecimal {
    fn write(&self, xml: &mut XmlWriter) -> Result<(), XmlError> {
        xml.write(&self.with_scale(4).to_string())
    }
}
