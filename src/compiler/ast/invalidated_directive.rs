use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Represents a directive that failed to parsed.
#[derive(Clone, Serialize, Deserialize)]
pub struct InvalidatedDirective {
    pub location: Location,
}