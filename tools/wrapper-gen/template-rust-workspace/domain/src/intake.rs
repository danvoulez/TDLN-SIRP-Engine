use anyhow::Result;
use crate::schema::InsuranceClaim;

pub enum Intake {
    Api(InsuranceClaim),
}

pub fn normalize(i: Intake) -> Result<InsuranceClaim> {
    match i {
        Intake::Api(c) => Ok(c),
    }
}
