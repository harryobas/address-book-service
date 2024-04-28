use serde::{Serialize, Deserialize};
use crate::types::address_book::AddressBookId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: ContactId,
    pub name: String,
    pub address: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address_book_id: AddressBookId
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ContactId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewContact {
    pub name: String,
    pub address: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address_book_id: AddressBookId
}