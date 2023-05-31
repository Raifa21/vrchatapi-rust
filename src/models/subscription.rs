/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// Subscription : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "steamItemId")]
    pub steam_item_id: String,
    #[serde(rename = "amount")]
    pub amount: f32,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "period")]
    pub period: crate::models::SubscriptionPeriod,
    #[serde(rename = "tier")]
    pub tier: f32,
}

impl Subscription {
    /// 
    pub fn new(id: String, steam_item_id: String, amount: f32, description: String, period: crate::models::SubscriptionPeriod, tier: f32) -> Subscription {
        Subscription {
            id,
            steam_item_id,
            amount,
            description,
            period,
            tier,
        }
    }
}


