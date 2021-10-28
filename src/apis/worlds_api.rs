/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorldError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorldError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_active_worlds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveWorldsError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_favorited_worlds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFavoritedWorldsError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_recent_worlds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRecentWorldsError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldError {
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_world_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldInstanceError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_world_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldMetadataError {
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_world_publish_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorldPublishStatusError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`publish_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishWorldError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_worlds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchWorldsError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unpublish_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpublishWorldError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorldError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Create a new world. This endpoint requires `assetUrl` to be a valid File object with `.vrcw` file extension, and `imageUrl` to be a valid File object with an image file extension.
pub fn create_world(configuration: &configuration::Configuration, create_world_request: Option<crate::models::CreateWorldRequest>) -> Result<crate::models::World, Error<CreateWorldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_world_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a world. Notice a world is never fully \"deleted\", only its ReleaseStatus is set to \"hidden\" and the linked Files are deleted. The WorldID is permanently reserved.
pub fn delete_world(configuration: &configuration::Configuration, world_id: &str) -> Result<(), Error<DeleteWorldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list currently Active worlds by query filters.
pub fn get_active_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<GetActiveWorldsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/active", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = featured {
        local_var_req_builder = local_var_req_builder.query(&[("featured", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = notag {
        local_var_req_builder = local_var_req_builder.query(&[("notag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = release_status {
        local_var_req_builder = local_var_req_builder.query(&[("releaseStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("maxUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("minUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetActiveWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list favorited worlds by query filters.
pub fn get_favorited_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>, user_id: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<GetFavoritedWorldsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/favorites", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = featured {
        local_var_req_builder = local_var_req_builder.query(&[("featured", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = notag {
        local_var_req_builder = local_var_req_builder.query(&[("notag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = release_status {
        local_var_req_builder = local_var_req_builder.query(&[("releaseStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("maxUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("minUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFavoritedWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list recently visited worlds by query filters.
pub fn get_recent_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>, user_id: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<GetRecentWorldsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/recent", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = featured {
        local_var_req_builder = local_var_req_builder.query(&[("featured", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = notag {
        local_var_req_builder = local_var_req_builder.query(&[("notag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = release_status {
        local_var_req_builder = local_var_req_builder.query(&[("releaseStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("maxUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("minUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRecentWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a specific World.
pub fn get_world(configuration: &configuration::Configuration, world_id: &str) -> Result<crate::models::World, Error<GetWorldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a worlds instance.
pub fn get_world_instance(configuration: &configuration::Configuration, world_id: &str, instance_id: &str) -> Result<crate::models::Instance, Error<GetWorldInstanceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/{instanceId}", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id), instanceId=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldInstanceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return a worlds custom metadata. This is currently believed to be unused. Metadata can be set with `updateWorld` and can be any arbitrary object.
pub fn get_world_metadata(configuration: &configuration::Configuration, world_id: &str) -> Result<crate::models::WorldMetadata, Error<GetWorldMetadataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/metadata", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a worlds publish status.
pub fn get_world_publish_status(configuration: &configuration::Configuration, world_id: &str) -> Result<crate::models::WorldPublishStatus, Error<GetWorldPublishStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/publish", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWorldPublishStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Publish a world. You can only publish one world per week.
pub fn publish_world(configuration: &configuration::Configuration, world_id: &str) -> Result<(), Error<PublishWorldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/publish", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PublishWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search and list any worlds by query filters.
pub fn search_worlds(configuration: &configuration::Configuration, featured: Option<&str>, sort: Option<&str>, user: Option<&str>, user_id: Option<&str>, n: Option<i32>, order: Option<&str>, offset: Option<i32>, search: Option<&str>, tag: Option<&str>, notag: Option<&str>, release_status: Option<&str>, max_unity_version: Option<&str>, min_unity_version: Option<&str>, platform: Option<&str>) -> Result<Vec<crate::models::LimitedWorld>, Error<SearchWorldsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = featured {
        local_var_req_builder = local_var_req_builder.query(&[("featured", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = n {
        local_var_req_builder = local_var_req_builder.query(&[("n", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag {
        local_var_req_builder = local_var_req_builder.query(&[("tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = notag {
        local_var_req_builder = local_var_req_builder.query(&[("notag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = release_status {
        local_var_req_builder = local_var_req_builder.query(&[("releaseStatus", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("maxUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_unity_version {
        local_var_req_builder = local_var_req_builder.query(&[("minUnityVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = platform {
        local_var_req_builder = local_var_req_builder.query(&[("platform", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchWorldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unpublish a world.
pub fn unpublish_world(configuration: &configuration::Configuration, world_id: &str) -> Result<(), Error<UnpublishWorldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}/publish", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UnpublishWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update information about a specific World.
pub fn update_world(configuration: &configuration::Configuration, world_id: &str, update_world_request: Option<crate::models::UpdateWorldRequest>) -> Result<crate::models::World, Error<UpdateWorldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/worlds/{worldId}", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_world_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateWorldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

