/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigConstantsInstancePopulationBrackets : Population brackets based on instance population
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigConstantsInstancePopulationBrackets {
    #[serde(rename = "CROWDED", skip_serializing_if = "Option::is_none")]
    pub crowded: Option<models::ApiConfigConstantsInstancePopulationBracketsCrowded>,
    #[serde(rename = "FEW", skip_serializing_if = "Option::is_none")]
    pub few: Option<models::ApiConfigConstantsInstancePopulationBracketsFew>,
    #[serde(rename = "MANY", skip_serializing_if = "Option::is_none")]
    pub many: Option<models::ApiConfigConstantsInstancePopulationBracketsMany>,
}

impl ApiConfigConstantsInstancePopulationBrackets {
    /// Population brackets based on instance population
    pub fn new() -> ApiConfigConstantsInstancePopulationBrackets {
        ApiConfigConstantsInstancePopulationBrackets {
            crowded: None,
            few: None,
            many: None,
        }
    }
}
