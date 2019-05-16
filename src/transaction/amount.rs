#[derive(Debug)]
pub struct Amount {
    pub vat_included: f64,
    pub vat_excluded: f64,
    pub vat_amount: f64,
}
