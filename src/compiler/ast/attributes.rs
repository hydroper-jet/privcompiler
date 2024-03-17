use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Attribute {
    Metadata(Rc<UnprocessedMetadata>),
    Public(Location),
    Private(Location),
    Protected(Location),
    Internal(Location),
    Proxy(Location),
    Final(Location),
    Native(Location),
    Static(Location),
    Abstract(Location),
    Override(Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UnprocessedMetadata {
    pub location: Location,
    pub name: (String, Location),
    pub entries: Option<Vec<Rc<UnprocessedMetadataEntry>>>,
}

impl UnprocessedMetadata {
    pub(crate) fn process(&self, verifier: &mut VerifierVerifier) -> Rc<Metadata> {
        let mut entries = Vec::<Rc<MetadataEntry>>::new();
        if let Some(u_entries) = self.entries.as_ref() {
            for entry in u_entries {
                let r = entry.process(verifier);
                if let Ok(r) = r {
                    entries.push(r.clone());
                }
            }
        }
        Rc::new(Metadata {
            name: self.name.0.clone(),
            entries,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UnprocessedMetadataEntry {
    pub location: Location,
    pub key: Option<(String, Location)>,
    pub value: Rc<UnprocessedMetadataValue>,
}

impl UnprocessedMetadataEntry {
    pub(crate) fn process(&self, verifier: &mut VerifierVerifier) -> Result<Rc<MetadataEntry>, ()> {
        let value = self.value.process(verifier);
        let Ok(value) = value else {
            return Err(());
        };
        Ok(Rc::new(MetadataEntry {
            key: self.key.as_ref().map(|(k, _)| k.clone()),
            value,
        }))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UnprocessedMetadataValue {
    IdentifierString((String, Location)),
    String((String, Location)),
    /// Numeric literal. Possibly contains a minus sign "-" prefix.
    Number((String, Location)),
    Boolean((bool, Location)),
    File {
        location: Location,
        output: bool,
        file_path: (String, Location),
    },
    List((Vec<Rc<UnprocessedMetadataEntry>>, Location)),
}

impl UnprocessedMetadataValue {
    pub fn location(&self) -> Location {
        match self {
            Self::IdentifierString((_, l)) => l.clone(),
            Self::String((_, l)) => l.clone(),
            Self::Number((_, l)) => l.clone(),
            Self::Boolean((_, l)) => l.clone(),
            Self::File { location, .. } => location.clone(),
            Self::List((_, l)) => l.clone(),
        }
    }

    pub(crate) fn process(&self, verifier: &mut VerifierVerifier) -> Result<Rc<MetadataValue>, ()> {
        match self {
            Self::IdentifierString((v, _)) => Ok(Rc::new(MetadataValue::String(v.clone()))),
            Self::String((v, _)) => Ok(Rc::new(MetadataValue::String(v.clone()))),
            Self::Number((v, l)) => {
                let mut v = v.clone();
                let mut negative = false;
                if v.starts_with("-") {
                    negative = true;
                    v = v[1..].to_owned();
                }
                let v = NumericLiteral {
                    value: v,
                    location: l.clone(),
                };
                let v = v.parse_double(negative);
                if let Ok(v) = v {
                    Ok(Rc::new(MetadataValue::Number(v)))
                } else {
                    verifier.add_verify_error(&l, DiagnosticKind::FailedParsingNumericLiteral, diagnostic_arguments![]);
                    Err(())
                }
            },
            Self::Boolean((v, _)) => Ok(Rc::new(MetadataValue::Boolean(*v))),
            Self::File { location, output, file_path } => {
                use file_paths::FlexPath;

                // Resolve file path
                let mut file_path = file_path.0.clone();
                if *output {
                    file_path = FlexPath::from_n_native([verifier.host.jetpm_output_directory().as_ref(), file_path.as_ref()]).to_string_with_flex_separator();
                } else {
                    file_path = FlexPath::new_native(&self.location().compilation_unit().file_path().unwrap_or(String::new())).resolve("..").resolve(&file_path).to_string_with_flex_separator();
                }

                // Read file
                if let Ok(data) = std::fs::read(&file_path) {
                    Ok(Rc::new(MetadataValue::File {
                        filename: FlexPath::new_native(&file_path).base_name(),
                        data,
                    }))
                } else {
                    verifier.add_verify_error(&location, DiagnosticKind::FailedLoadingMetadataFile, diagnostic_arguments![String(file_path)]);
                    Err(())
                }
            },
            Self::List((u_entries, _)) => {
                let mut entries = Vec::<Rc<MetadataEntry>>::new();
                for entry in u_entries {
                    let r = entry.process(verifier);
                    if let Ok(r) = r {
                        entries.push(r.clone());
                    }
                }
                Ok(Rc::new(MetadataValue::List(entries)))
            },
        }
    }
}

impl Attribute {
    pub fn location(&self) -> Location {
        match self {
            Self::Metadata(m) => m.location.clone(),
            Self::Public(a) => a.clone(),
            Self::Private(a) => a.clone(),
            Self::Protected(a) => a.clone(),
            Self::Internal(a) => a.clone(),
            Self::Proxy(a) => a.clone(),
            Self::Final(a) => a.clone(),
            Self::Native(a) => a.clone(),
            Self::Static(a) => a.clone(),
            Self::Abstract(a) => a.clone(),
            Self::Override(a) => a.clone(),
        }
    }

    pub fn visibility(list: &Vec<Attribute>, at_interface_block: bool) -> Visibility {
        if at_interface_block {
            return Visibility::Public;
        }
        for a in list {
            match a {
                Self::Public(_) => return Visibility::Public,
                Self::Private(_) => return Visibility::Private,
                Self::Protected(_) => return Visibility::Protected,
                Self::Internal(_) => return Visibility::Internal,
                _ => {}
            }
        }
        Visibility::Internal
    }

    pub fn has_visibility(list: &Vec<Attribute>) -> bool {
        for a in list {
            match a {
                Self::Public(_) |
                Self::Private(_) |
                Self::Protected(_) |
                Self::Internal(_) => return true,
                _ => {}
            }
        }
        false
    }

    pub fn remove_metadata(list: &mut Vec<Attribute>, metadata: &Rc<UnprocessedMetadata>) {
        for i in 0..list.len() {
            if let Attribute::Metadata(metadata_1) = &list[i] {
                if Rc::ptr_eq(&metadata_1, metadata) {
                    list.remove(i);
                    break;
                }
            }
        }
    }

    pub fn find_metadata(list: &Vec<Attribute>) -> Vec<Rc<UnprocessedMetadata>> {
        let mut r = vec![];
        for a in list {
            match &a {
                Self::Metadata(e) => {
                    r.push(e.clone());
                },
                _ => {},
            }
        }
        r
    }
    pub fn find_public(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Public(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_private(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Private(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_protected(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Protected(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_internal(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Internal(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_proxy(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Proxy(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_final(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Final(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_native(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Native(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_static(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Static(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_abstract(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Abstract(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_override(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Override(l) => return Some(l.clone()), _ => return None } }; None }

    pub fn has(list: &Vec<Attribute>, attribute: &Attribute) -> bool {
        match attribute {
            Self::Public(_) => Self::find_public(list).is_some(),
            Self::Private(_) => Self::find_private(list).is_some(),
            Self::Protected(_) => Self::find_protected(list).is_some(),
            Self::Internal(_) => Self::find_internal(list).is_some(),
            Self::Proxy(_) => Self::find_proxy(list).is_some(),
            Self::Final(_) => Self::find_final(list).is_some(),
            Self::Native(_) => Self::find_native(list).is_some(),
            Self::Static(_) => Self::find_static(list).is_some(),
            Self::Abstract(_) => Self::find_abstract(list).is_some(),
            Self::Override(_) => Self::find_override(list).is_some(),
            _ => false,
        }
    }

    pub fn is_duplicate_visibility(list: &Vec<Attribute>, attribute: &Attribute) -> bool {
        match attribute {
            Self::Public(_) |
            Self::Private(_) |
            Self::Protected(_) |
            Self::Internal(_) => Self::find_public(list).is_some() || Self::find_private(list).is_some() || Self::find_protected(list).is_some() || Self::find_internal(list).is_some(),
            _ => false,
        }
    }

    pub fn is_metadata(&self) -> bool { matches!(self, Self::Metadata(_)) }
    pub fn is_public(&self) -> bool { matches!(self, Self::Public(_)) }
    pub fn is_private(&self) -> bool { matches!(self, Self::Private(_)) }
    pub fn is_protected(&self) -> bool { matches!(self, Self::Protected(_)) }
    pub fn is_internal(&self) -> bool { matches!(self, Self::Internal(_)) }
    pub fn is_proxy(&self) -> bool { matches!(self, Self::Proxy(_)) }
    pub fn is_final(&self) -> bool { matches!(self, Self::Final(_)) }
    pub fn is_native(&self) -> bool { matches!(self, Self::Native(_)) }
    pub fn is_static(&self) -> bool { matches!(self, Self::Static(_)) }
    pub fn is_abstract(&self) -> bool { matches!(self, Self::Abstract(_)) }
    pub fn is_override(&self) -> bool { matches!(self, Self::Override(_)) }

    pub fn from_identifier_name(name: &str, location: &Location) -> Option<Attribute> {
        if location.character_count() != name.chars().count() {
            return None;
        }
        match name.as_ref() {
            "proxy" => Some(Attribute::Proxy(location.clone())),
            "final" => Some(Attribute::Final(location.clone())),
            "native" => Some(Attribute::Native(location.clone())),
            "static" => Some(Attribute::Static(location.clone())),
            "abstract" => Some(Attribute::Abstract(location.clone())),
            "override" => Some(Attribute::Override(location.clone())),
            _ => None,
        }
    }
}