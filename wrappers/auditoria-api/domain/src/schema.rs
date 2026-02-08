use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct InsuranceClaim {
    #[validate(length(min = 1, max = 100))]
    pub claim_id: String,
    pub amount: Decimal,
    pub incident_date: DateTime<Utc>,
    pub evidence_cids: Vec<String>,
}
