/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// TransactionAgreement : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionAgreement {
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "itemId")]
    pub item_id: f32,
    /// This is NOT TransactionStatus, but whatever Steam return.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "frequency")]
    pub frequency: f32,
    #[serde(rename = "billingType")]
    pub billing_type: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "recurringAmt")]
    pub recurring_amt: f32,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "nextPayment")]
    pub next_payment: String,
    #[serde(rename = "lastPayment")]
    pub last_payment: String,
    #[serde(rename = "lastAmount")]
    pub last_amount: f32,
    #[serde(rename = "lastAmountVat")]
    pub last_amount_vat: f32,
    #[serde(rename = "outstanding")]
    pub outstanding: f32,
    #[serde(rename = "failedAttempts")]
    pub failed_attempts: f32,
}

impl TransactionAgreement {
    /// 
    pub fn new(agreement_id: String, item_id: f32, status: String, period: String, frequency: f32, billing_type: String, start_date: String, end_date: String, recurring_amt: f32, currency: String, time_created: String, next_payment: String, last_payment: String, last_amount: f32, last_amount_vat: f32, outstanding: f32, failed_attempts: f32) -> TransactionAgreement {
        TransactionAgreement {
            agreement_id,
            item_id,
            status,
            period,
            frequency,
            billing_type,
            start_date,
            end_date,
            recurring_amt,
            currency,
            time_created,
            next_payment,
            last_payment,
            last_amount,
            last_amount_vat,
            outstanding,
            failed_attempts,
        }
    }
}


