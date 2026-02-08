use crate::schema::InsuranceClaim;
use anyhow::Result;

pub enum Intake {
    Api(InsuranceClaim),
}

pub fn normalize(i: Intake) -> Result<InsuranceClaim> {
    match i {
        Intake::Api(c) => Ok(c),
    }
}
