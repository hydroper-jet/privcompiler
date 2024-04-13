use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UsePackageDirective {
    pub location: Location,
    pub alias: Option<(String, Location)>,
    pub package_name: Vec<(String, Location)>,
    pub import_specifier: ImportSpecifier,
}