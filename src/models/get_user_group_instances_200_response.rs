/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserGroupInstances200Response {
    #[serde(rename = "fetchedAt", skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<String>,
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<models::Instance>>,
}

impl GetUserGroupInstances200Response {
    pub fn new() -> GetUserGroupInstances200Response {
        GetUserGroupInstances200Response {
            fetched_at: None,
            instances: None,
        }
    }
}
