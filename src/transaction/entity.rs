use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub label: String,
    pub code: String,
}

impl Entity {

    pub fn new(
        code: String,
        label: String,
    ) -> Entity {
        Self {
            label,
            code,
        }
    }

}