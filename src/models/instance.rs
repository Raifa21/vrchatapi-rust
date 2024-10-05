/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Instance : * `hidden` field is only present if InstanceType is `hidden` aka \"Friends+\", and is instance creator. * `friends` field is only present if InstanceType is `friends` aka \"Friends\", and is instance creator. * `private` field is only present if InstanceType is `private` aka \"Invite\" or \"Invite+\", and is instance creator.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "canRequestInvite")]
    pub can_request_invite: bool,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    /// Always returns \"unknown\".
    #[serde(rename = "clientNumber")]
    pub client_number: String,
    #[serde(rename = "displayName", deserialize_with = "Option::deserialize")]
    pub display_name: Option<String>,
    #[serde(rename = "full")]
    pub full: bool,
    #[serde(rename = "gameServerVersion")]
    pub game_server_version: i32,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(
        rename = "instancePersistenceEnabled",
        deserialize_with = "Option::deserialize"
    )]
    pub instance_persistence_enabled: Option<String>,
    /// InstanceID can be \"offline\" on User profiles if you are not friends with that user and \"private\" if you are friends and user is in private instance.
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "n_users")]
    pub n_users: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// A groupId if the instance type is \"group\", null if instance type is public, or a userId otherwise
    #[serde(
        rename = "ownerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_id: Option<Option<String>>,
    #[serde(rename = "permanent")]
    pub permanent: bool,
    #[serde(rename = "photonRegion")]
    pub photon_region: models::Region,
    #[serde(rename = "platforms")]
    pub platforms: models::InstancePlatforms,
    #[serde(
        rename = "playerPersistenceEnabled",
        deserialize_with = "Option::deserialize"
    )]
    pub player_persistence_enabled: Option<String>,
    #[serde(rename = "region")]
    pub region: models::InstanceRegion,
    #[serde(rename = "secureName")]
    pub secure_name: String,
    #[serde(
        rename = "shortName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<Option<String>>,
    /// The tags array on Instances usually contain the language tags of the people in the instance.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: models::InstanceType,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "worldId")]
    pub world_id: String,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "friends", skip_serializing_if = "Option::is_none")]
    pub friends: Option<String>,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<String>,
    #[serde(rename = "queueEnabled")]
    pub queue_enabled: bool,
    #[serde(rename = "queueSize")]
    pub queue_size: i32,
    #[serde(rename = "recommendedCapacity")]
    pub recommended_capacity: i32,
    #[serde(rename = "roleRestricted", skip_serializing_if = "Option::is_none")]
    pub role_restricted: Option<bool>,
    #[serde(rename = "strict")]
    pub strict: bool,
    #[serde(rename = "userCount")]
    pub user_count: i32,
    #[serde(rename = "world")]
    pub world: models::World,
    /// The users field is present on instances created by the requesting user.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::LimitedUser>>,
    #[serde(rename = "groupAccessType", skip_serializing_if = "Option::is_none")]
    pub group_access_type: Option<models::GroupAccessType>,
    #[serde(rename = "hasCapacityForYou", skip_serializing_if = "Option::is_none")]
    pub has_capacity_for_you: Option<bool>,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(
        rename = "closedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub closed_at: Option<Option<String>>,
    #[serde(
        rename = "hardClose",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hard_close: Option<Option<bool>>,
}

impl Instance {
    /// * `hidden` field is only present if InstanceType is `hidden` aka \"Friends+\", and is instance creator. * `friends` field is only present if InstanceType is `friends` aka \"Friends\", and is instance creator. * `private` field is only present if InstanceType is `private` aka \"Invite\" or \"Invite+\", and is instance creator.
    pub fn new(
        active: bool,
        can_request_invite: bool,
        capacity: i32,
        client_number: String,
        display_name: Option<String>,
        full: bool,
        game_server_version: i32,
        id: String,
        instance_id: String,
        instance_persistence_enabled: Option<String>,
        location: String,
        n_users: i32,
        name: String,
        permanent: bool,
        photon_region: models::Region,
        platforms: models::InstancePlatforms,
        player_persistence_enabled: Option<String>,
        region: models::InstanceRegion,
        secure_name: String,
        tags: Vec<String>,
        r#type: models::InstanceType,
        world_id: String,
        queue_enabled: bool,
        queue_size: i32,
        recommended_capacity: i32,
        strict: bool,
        user_count: i32,
        world: models::World,
    ) -> Instance {
        Instance {
            active,
            can_request_invite,
            capacity,
            client_number,
            display_name,
            full,
            game_server_version,
            id,
            instance_id,
            instance_persistence_enabled,
            location,
            n_users,
            name,
            owner_id: None,
            permanent,
            photon_region,
            platforms,
            player_persistence_enabled,
            region,
            secure_name,
            short_name: None,
            tags,
            r#type,
            world_id,
            hidden: None,
            friends: None,
            private: None,
            queue_enabled,
            queue_size,
            recommended_capacity,
            role_restricted: None,
            strict,
            user_count,
            world,
            users: None,
            group_access_type: None,
            has_capacity_for_you: None,
            nonce: None,
            closed_at: None,
            hard_close: None,
        }
    }
}
