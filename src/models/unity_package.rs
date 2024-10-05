/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UnityPackage :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnityPackage {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "assetUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_url: Option<Option<String>>,
    #[serde(rename = "assetUrlObject", skip_serializing_if = "Option::is_none")]
    pub asset_url_object: Option<serde_json::Value>,
    #[serde(rename = "assetVersion")]
    pub asset_version: i32,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(
        rename = "impostorizerVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub impostorizer_version: Option<String>,
    #[serde(rename = "performanceRating", skip_serializing_if = "Option::is_none")]
    pub performance_rating: Option<models::PerformanceRatings>,
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "pluginUrl", skip_serializing_if = "Option::is_none")]
    pub plugin_url: Option<String>,
    #[serde(rename = "pluginUrlObject", skip_serializing_if = "Option::is_none")]
    pub plugin_url_object: Option<serde_json::Value>,
    #[serde(rename = "unitySortNumber", skip_serializing_if = "Option::is_none")]
    pub unity_sort_number: Option<i64>,
    #[serde(rename = "unityVersion")]
    pub unity_version: String,
    #[serde(
        rename = "worldSignature",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub world_signature: Option<Option<String>>,
    #[serde(
        rename = "impostorUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub impostor_url: Option<Option<String>>,
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl UnityPackage {
    pub fn new(
        id: String,
        asset_version: i32,
        platform: String,
        unity_version: String,
    ) -> UnityPackage {
        UnityPackage {
            id,
            asset_url: None,
            asset_url_object: None,
            asset_version,
            created_at: None,
            impostorizer_version: None,
            performance_rating: None,
            platform,
            plugin_url: None,
            plugin_url_object: None,
            unity_sort_number: None,
            unity_version,
            world_signature: None,
            impostor_url: None,
            scan_status: None,
            variant: None,
        }
    }
}
