/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ApiConfigReportReasons : Reasons available for reporting users
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiConfigReportReasons {
    #[serde(rename = "billing")]
    pub billing: models::ReportReason,
    #[serde(rename = "botting")]
    pub botting: models::ReportReason,
    #[serde(rename = "cancellation")]
    pub cancellation: models::ReportReason,
    #[serde(rename = "gore")]
    pub gore: models::ReportReason,
    #[serde(rename = "hacking")]
    pub hacking: models::ReportReason,
    #[serde(rename = "harassing")]
    pub harassing: models::ReportReason,
    #[serde(rename = "hateful")]
    pub hateful: models::ReportReason,
    #[serde(rename = "impersonation")]
    pub impersonation: models::ReportReason,
    #[serde(rename = "inappropriate")]
    pub inappropriate: models::ReportReason,
    #[serde(rename = "leaking")]
    pub leaking: models::ReportReason,
    #[serde(rename = "malicious")]
    pub malicious: models::ReportReason,
    #[serde(rename = "missing")]
    pub missing: models::ReportReason,
    #[serde(rename = "nudity")]
    pub nudity: models::ReportReason,
    #[serde(rename = "renewal")]
    pub renewal: models::ReportReason,
    #[serde(rename = "security")]
    pub security: models::ReportReason,
    #[serde(rename = "service")]
    pub service: models::ReportReason,
    #[serde(rename = "sexual")]
    pub sexual: models::ReportReason,
    #[serde(rename = "threatening")]
    pub threatening: models::ReportReason,
    #[serde(rename = "visuals")]
    pub visuals: models::ReportReason,
}

impl ApiConfigReportReasons {
    /// Reasons available for reporting users
    pub fn new(
        billing: models::ReportReason,
        botting: models::ReportReason,
        cancellation: models::ReportReason,
        gore: models::ReportReason,
        hacking: models::ReportReason,
        harassing: models::ReportReason,
        hateful: models::ReportReason,
        impersonation: models::ReportReason,
        inappropriate: models::ReportReason,
        leaking: models::ReportReason,
        malicious: models::ReportReason,
        missing: models::ReportReason,
        nudity: models::ReportReason,
        renewal: models::ReportReason,
        security: models::ReportReason,
        service: models::ReportReason,
        sexual: models::ReportReason,
        threatening: models::ReportReason,
        visuals: models::ReportReason,
    ) -> ApiConfigReportReasons {
        ApiConfigReportReasons {
            billing,
            botting,
            cancellation,
            gore,
            hacking,
            harassing,
            hateful,
            impersonation,
            inappropriate,
            leaking,
            malicious,
            missing,
            nudity,
            renewal,
            security,
            service,
            sexual,
            threatening,
            visuals,
        }
    }
}
