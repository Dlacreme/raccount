use super::transaction;
use chrono::prelude::*;

pub fn transaction(code: String) -> transaction::Transaction {
    transaction::Transaction::new(
        String::from(code),
        String::from("{}"),
        amount(),
        amount(),
        "EUR".as_bytes(),
        entity(String::from("GENESIS FROM")),
        entity(String::from("GENESIS TO")),
        Utc::now()
    )
}

pub fn amount() -> transaction::amount::Amount {
    transaction::amount::Amount::new(0.0, 0.0, 0.0)
}

pub fn entity(label: String) -> transaction::entity::Entity {
    transaction::entity::Entity::new(
        label.clone(),
        label.clone()
    )
}