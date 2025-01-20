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
pub struct GroupRole {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isSelfAssignable", skip_serializing_if = "Option::is_none")]
    pub is_self_assignable: Option<bool>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permissions>>,
    #[serde(rename = "isManagementRole", skip_serializing_if = "Option::is_none")]
    pub is_management_role: Option<bool>,
    #[serde(rename = "requiresTwoFactor", skip_serializing_if = "Option::is_none")]
    pub requires_two_factor: Option<bool>,
    #[serde(rename = "requiresPurchase", skip_serializing_if = "Option::is_none")]
    pub requires_purchase: Option<bool>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl GroupRole {
    pub fn new() -> GroupRole {
        GroupRole {
            id: None,
            group_id: None,
            name: None,
            description: None,
            is_self_assignable: None,
            permissions: None,
            is_management_role: None,
            requires_two_factor: None,
            requires_purchase: None,
            order: None,
            created_at: None,
            updated_at: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "\\*")]
    group_all,
    #[serde(rename = "group-announcement-manage")]
    group_announcement_manage,
    #[serde(rename = "group-audit-view")]
    group_audit_view,
    #[serde(rename = "group-bans-manage")]
    group_bans_manage,
    #[serde(rename = "group-data-manage")]
    group_data_manage,
    #[serde(rename = "group-default-role-manage")]
    group_default_role_manage,
    #[serde(rename = "group-galleries-manage")]
    group_galleries_manage,
    #[serde(rename = "group-instance-age-gated-create")]
    group_instance_age_gated_create,
    #[serde(rename = "group-instance-join")]
    group_instance_join,
    #[serde(rename = "group-instance-manage")]
    group_instance_manage,
    #[serde(rename = "group-instance-moderate")]
    group_instance_moderate,
    #[serde(rename = "group-instance-open-create")]
    group_instance_open_create,
    #[serde(rename = "group-instance-plus-create")]
    group_instance_plus_create,
    #[serde(rename = "group-instance-plus-portal")]
    group_instance_plus_portal,
    #[serde(rename = "group-instance-plus-portal-unlocked")]
    group_instance_plus_portal_unlocked,
    #[serde(rename = "group-instance-public-create")]
    group_instance_public_create,
    #[serde(rename = "group-instance-queue-priority")]
    group_instance_queue_priority,
    #[serde(rename = "group-instance-restricted-create")]
    group_instance_restricted_create,
    #[serde(rename = "group-invites-manage")]
    group_invites_manage,
    #[serde(rename = "group-members-manage")]
    group_members_manage,
    #[serde(rename = "group-members-remove")]
    group_members_remove,
    #[serde(rename = "group-members-viewall")]
    group_members_viewall,
    #[serde(rename = "group-roles-assign")]
    group_roles_assign,
    #[serde(rename = "group-roles-manage")]
    group_roles_manage,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Self::group_all
    }
}
