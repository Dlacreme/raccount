use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub vat_included: f64,
    pub vat_excluded: f64,
    pub vat_amount: f64,
}

impl Amount {

    pub fn new(included: f64, excluded: f64, vat_amount: f64) -> Self {
        Self {
            vat_included: included,
            vat_excluded: excluded,
            vat_amount: vat_amount,
        }
    }

}