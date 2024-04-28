use serde::{Serialize, Deserialize};
use crate::types::contact::Contact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBook {
    pub id: AddressBookId,
    pub address_book_name: String,
    pub contacts: Vec<Contact>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AddressBookId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAddressBook {
    pub address_book_name: String
}