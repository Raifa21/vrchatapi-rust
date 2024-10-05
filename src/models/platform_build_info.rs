/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PlatformBuildInfo : Build information for a platform
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformBuildInfo {
    /// Minimum build number required for the platform
    #[serde(rename = "minBuildNumber")]
    pub min_build_number: i32,
    /// Redirection URL for updating the app
    #[serde(rename = "redirectionAddress")]
    pub redirection_address: String,
}

impl PlatformBuildInfo {
    /// Build information for a platform
    pub fn new(min_build_number: i32, redirection_address: String) -> PlatformBuildInfo {
        PlatformBuildInfo {
            min_build_number,
            redirection_address,
        }
    }
}
