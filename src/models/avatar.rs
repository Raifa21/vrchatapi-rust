/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Avatar : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    /// Not present from general serach `/avatars`, only on specific requests `/avatars/{avatarId}`.
    #[serde(rename = "assetUrl", skip_serializing_if = "Option::is_none")]
    pub asset_url: Option<String>,
    /// Not present from general serach `/avatars`, only on specific requests `/avatars/{avatarId}`. **Deprecation:** `Object` has unknown usage/fields, and is always empty. Use normal `Url` field instead.
    #[serde(rename = "assetUrlObject", skip_serializing_if = "Option::is_none")]
    pub asset_url_object: Option<serde_json::Value>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "featured")]
    pub featured: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "releaseStatus")]
    pub release_status: models::ReleaseStatus,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "thumbnailImageUrl")]
    pub thumbnail_image_url: String,
    #[serde(rename = "unityPackageUrl")]
    pub unity_package_url: String,
    #[serde(rename = "unityPackageUrlObject")]
    pub unity_package_url_object: models::AvatarUnityPackageUrlObject,
    #[serde(rename = "unityPackages")]
    pub unity_packages: Vec<models::UnityPackage>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "version")]
    pub version: i32,
}

impl Avatar {
    pub fn new(author_id: String, author_name: String, created_at: String, description: String, featured: bool, id: String, image_url: String, name: String, release_status: models::ReleaseStatus, tags: Vec<String>, thumbnail_image_url: String, unity_package_url: String, unity_package_url_object: models::AvatarUnityPackageUrlObject, unity_packages: Vec<models::UnityPackage>, updated_at: String, version: i32) -> Avatar {
        Avatar {
            asset_url: None,
            asset_url_object: None,
            author_id,
            author_name,
            created_at,
            description,
            featured,
            id,
            image_url,
            name,
            release_status,
            tags,
            thumbnail_image_url,
            unity_package_url,
            unity_package_url_object,
            unity_packages,
            updated_at,
            version,
        }
    }
}

