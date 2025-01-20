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
pub struct TokenBundle {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "steamItemId")]
    pub steam_item_id: String,
    #[serde(rename = "oculusSku")]
    pub oculus_sku: String,
    /// price of the bundle
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "description")]
    pub description: String,
    /// number of tokens received
    #[serde(rename = "tokens")]
    pub tokens: i32,
    /// direct url to image
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

impl TokenBundle {
    pub fn new(
        id: String,
        steam_item_id: String,
        oculus_sku: String,
        amount: i32,
        description: String,
        tokens: i32,
        image_url: String,
    ) -> TokenBundle {
        TokenBundle {
            id,
            steam_item_id,
            oculus_sku,
            amount,
            description,
            tokens,
            image_url,
        }
    }
}
