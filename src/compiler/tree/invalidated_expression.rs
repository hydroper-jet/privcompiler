use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Represents an expression that failed to parsed.
#[derive(Clone, Serialize, Deserialize)]
pub struct InvalidatedExpression {
    pub location: Location,
}