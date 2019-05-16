mod amount;
mod entity;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub data: String,
    pub amount: amount::Amount,
    pub original_amount: amount::Amount,
    pub original_currency: Vec<char>,
    pub from: entity::Entity,
    pub to: entity::Entity,
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(
        id: String,
        data: String,
        amount: amount::Amount, original_amount: amount::Amount, original_currency: Vec<char>,
        from: entity::Entity, to: entity::Entity,
        date: chrono::DateTime<chrono::Utc>
    ) -> Self {
        Self {
            id,
            data,
            amount,
            original_amount,
            original_currency,
            from,
            to,
            timestamp: date.timestamp()
        }
    }
}

