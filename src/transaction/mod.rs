
#[derive(Debug)]
pub struct Transaction {
    pub id: String,
}

impl Transaction {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
        }
    }
}

