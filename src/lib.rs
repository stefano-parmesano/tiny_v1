mod error;

use crate::error::TinyV1Error;

pub struct Header<'a> {
    pub namespace_a: &'a str,
    pub namespace_b: &'a str,
    pub namespaces: Vec<&'a str>,
}

impl<'a> Header<'a> {
    fn from_str<'b>(s: &'a str) -> Result<Self, TinyV1Error<'b>> {
        let mut header = s.split('\t');
        if header.next().ok_or(TinyV1Error("Empty Header"))? != "v1" {
            return Err(TinyV1Error("Wrong Format"));
        }

        let namespace_a = header.next().ok_or(TinyV1Error("Namespace a not found"))?;
        let namespace_b = header.next().ok_or(TinyV1Error("Namespace b not found"))?;
        let namespaces: Vec<&'a str> = header.collect();
        Ok(Self {
            namespace_a,
            namespace_b,
            namespaces,
        })
    }
}

pub struct Content<'a> {
    pub mapping_entries: Vec<MappingEntry<'a>>,
}

pub enum MappingEntry<'a> {
    CLASS {
        class_names: Vec<&'a str>,
    },
    FIELD {
        parent_class_name_a: &'a str,
        field_desc_a: &'a str,
        field_name_a: &'a str,
        field_name_b: &'a str,
        extra_ns_field_names: Vec<&'a str>,
    },
    METHOD {
        parent_class_name_a: &'a str,
        method_desc_a: &'a str,
        method_name_a: &'a str,
        method_name_b: &'a str,
        extra_ns_method_names: Vec<&'a str>,
    },
}

impl<'a> MappingEntry<'a> {
    fn from_str<'b>(s: &'a str) -> Result<Self, TinyV1Error<'b>> {
        let mut mapping_entry = s.split('\t');
        match mapping_entry.next() {
            Some("CLASS") => {
                let class_names: Vec<_> = mapping_entry.collect();
                Ok(Self::CLASS { class_names })
            }
            Some("FIELD") => {
                let parent_class_name_a = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Parent class not found"))?;
                let field_desc_a = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Method desc not found"))?;
                let field_name_a = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Method name a not found"))?;
                let field_name_b = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Method name b not found"))?;
                let extra_ns_field_names: Vec<_> = mapping_entry.collect();
                Ok(Self::FIELD {
                    parent_class_name_a,
                    field_desc_a,
                    field_name_a,
                    field_name_b,
                    extra_ns_field_names,
                })
            }
            Some("METHOD") => {
                let parent_class_name_a = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Parent class not found"))?;
                let method_desc_a = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Method desc not found"))?;
                let method_name_a = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Method name a not found"))?;
                let method_name_b = mapping_entry
                    .next()
                    .ok_or(TinyV1Error("Method name b not found"))?;
                let extra_ns_method_names: Vec<_> = mapping_entry.collect();
                Ok(Self::METHOD {
                    parent_class_name_a,
                    method_desc_a,
                    method_name_a,
                    method_name_b,
                    extra_ns_method_names,
                })
            }
            None => Err(TinyV1Error("Line can not be empty")),
            _ => Err(TinyV1Error("Invalid Identifier")),
        }
    }
}

impl<'a> Content<'a> {
    fn from_str<'b>(s: &'a str) -> Result<Self, TinyV1Error<'b>> {
        let mapping_entries: Result<Vec<_>, _> = s
            .lines()
            .map(|mapping_entry| MappingEntry::from_str(mapping_entry))
            .collect();
        let mapping_entries = mapping_entries?;
        Ok(Self { mapping_entries })
    }
}

pub struct File<'a> {
    pub header: Header<'a>,
    pub content: Content<'a>,
}

impl<'a> File<'a> {
    pub fn from_str<'b>(s: &'a str) -> Result<Self, TinyV1Error<'b>> {
        let (header_str, content_str) = s
            .split_once('\n')
            .ok_or(TinyV1Error("There is no line break"))?;
        let header = Header::from_str(header_str)?;
        let content = Content::from_str(content_str)?;
        Ok(Self { header, content })
    }
}

pub fn from_str<'b>(s: &str) -> Result<File<'_>, TinyV1Error<'b>> {
    File::from_str(s)
}
