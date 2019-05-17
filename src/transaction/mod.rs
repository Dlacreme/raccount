use std::convert::AsMut;
use serde_derive::{Serialize, Deserialize};

pub mod amount;
pub mod entity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: String,
    pub data: String,
    pub amount: amount::Amount,
    pub original_amount: amount::Amount,
    pub original_currency: [u8; 3],
    pub from: entity::Entity,
    pub to: entity::Entity,
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(
        id: String,
        data: String,
        amount: amount::Amount, original_amount: amount::Amount,
        original_currency: &[u8],
        from: entity::Entity, to: entity::Entity,
        date: chrono::DateTime<chrono::Utc>
    ) -> Self {

        Self {
            id,
            data,
            amount,
            original_amount,
            original_currency: clone_into_array(original_currency),
            from,
            to,
            timestamp: date.timestamp()
        }

    }

}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}