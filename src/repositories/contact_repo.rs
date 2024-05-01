use crate::types::address_book::AddressBookId;
use crate::types::contact::{Contact, ContactId};

use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::PgPool;
use sqlx::Row;
#[async_trait]
pub trait IContactRepository {
    async fn get_address_book_contacts(
        &self,
        address_book_id: i32,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Contact>, handle_errors::Error>;

    async fn add_contact_to_address_book(
        &self,
        name: &str,
        address: &str,
        phone_number: Option<String>,
        email: Option<String>,
        address_book_id: i32,
    ) -> Result<Contact, handle_errors::Error>;

    async fn get_contact_by_id(
        &self,
        id: i32,
        address_book_id: i32,
    ) -> Result<Option<Contact>, handle_errors::Error>;

    async fn update_contact(
        &self,
        id: i32,
        name: &str,
        address: &str,
        phone_number: Option<String>,
        email: Option<String>,
        address_book_id: i32,
    ) -> Result<Contact, handle_errors::Error>;

    async fn delete_contact(
        &self,
        id: i32,
        address_book_id: i32,
    ) -> Result<(), handle_errors::Error>;

    //async fn find_contact_by_name(
    //  &self,
    //name: &str,
    //address_book_id: i32,
    // ) -> Result<Option<Contact>, handle_errors::Error>;
}

pub struct ContactRepository {
    pub pool: PgPool,
}

impl ContactRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IContactRepository for ContactRepository {
    async fn get_address_book_contacts(
        &self,
        address_book_id: i32,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Contact>, handle_errors::Error> {
        let q = "SELECT * FROM contacts WHERE address_book_id = $1 LIMIT $2 OFFSET $3";
        match sqlx::query(q)
            .bind(address_book_id)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                address: row.get("address"),
                phone_number: row.get("phone_number"),
                email: row.get("email"),
                address_book_id: AddressBookId(row.get("address_book_id")),
            })
            .fetch_all(&self.pool)
            .await
        {
            Ok(contacts) => Ok(contacts),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn add_contact_to_address_book(
        &self,
        name: &str,
        address: &str,
        phone_number: Option<String>,
        email: Option<String>,
        address_book_id: i32,
    ) -> Result<Contact, handle_errors::Error> {
        let q = "INSERT INTO address_books 
                      (name address phone_number email address_book_id)
                      VALUES ($1 $2 $3 $4 $5)";
        match sqlx::query(q)
            .bind(name)
            .bind(address)
            .bind(phone_number)
            .bind(email)
            .bind(address_book_id)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                address: row.get("address"),
                phone_number: row.get("phone_number"),
                email: row.get("email"),
                address_book_id: AddressBookId(row.get("address_book_id")),
            })
            .fetch_one(&self.pool)
            .await
        {
            Ok(contact) => Ok(contact),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn get_contact_by_id(
        &self,
        id: i32,
        address_book_id: i32,
    ) -> Result<Option<Contact>, handle_errors::Error> {
        let q = "SELECT * FROM contacts
                             WHERE id = $1 AND address_book_id = $2";
        match sqlx::query(q)
            .bind("id")
            .bind("address_book_id")
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                address: row.get("address"),
                phone_number: row.get("phone_number"),
                email: row.get("email"),
                address_book_id: AddressBookId(row.get("address_book_id")),
            })
            .fetch_optional(&self.pool)
            .await
        {
            Ok(contact) => Ok(contact),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn delete_contact(
        &self,
        id: i32,
        address_book_id: i32,
    ) -> Result<(), handle_errors::Error> {
        let q = "DELETE FROM contacts 
                                       WHERE id = $1 AND address_book_id = $2";
        match sqlx::query(q)
            .bind(id)
            .bind(address_book_id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn update_contact(
        &self,
        id: i32,
        name: &str,
        address: &str,
        phone_number: Option<String>,
        email: Option<String>,
        address_book_id: i32,
    ) -> Result<Contact, handle_errors::Error> {
        let q = "UPDATE contacts SET 
                                     name = $1, address = $2, phone_number = $3, email = $4 
                                     WHERE id = $5 AND address_book_id = $6";
        match sqlx::query(q)
            .bind(name)
            .bind(address)
            .bind(phone_number)
            .bind(email)
            .bind(id)
            .bind(address_book_id)
            .map(|row: PgRow| Contact {
                id: ContactId(row.get("id")),
                name: row.get("name"),
                address: row.get("address"),
                phone_number: row.get("phone_number"),
                email: row.get("email"),
                address_book_id: AddressBookId(row.get("address_book_id")),
            })
            .fetch_one(&self.pool)
            .await
        {
            Ok(contact) => Ok(contact),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }
}
