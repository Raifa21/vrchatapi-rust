/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigConstantsInstancePopulationBracketsMany : Many population range
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstancePopulationBracketsMany {
    /// Maximum population for a many instance
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Minimum population for a many instance
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl ApiConfigConstantsInstancePopulationBracketsMany {
    /// Many population range
    pub fn new() -> ApiConfigConstantsInstancePopulationBracketsMany {
        ApiConfigConstantsInstancePopulationBracketsMany {
            max: None,
            min: None,
        }
    }
}
