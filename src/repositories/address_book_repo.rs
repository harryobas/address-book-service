use crate::types::address_book::{AddressBook, AddressBookId};
use crate::types::contact::{Contact, ContactId};
use async_trait::async_trait;

use sqlx::postgres::PgRow;
use sqlx::Row;

#[async_trait]
pub trait IAddressBookRepository {
    async fn get_all_address_books(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<AddressBook>, handle_errors::Error>;

    async fn get_address_book_by_id(
        &self,
        id: i32,
    ) -> Result<Option<AddressBook>, handle_errors::Error>;
    async fn create_address_book(
        &self,
        address_book_name: &str,
    ) -> Result<AddressBook, handle_errors::Error>;
    async fn find_address_book_by_name(
        &self,
        name: &str,
    ) -> Result<Option<AddressBook>, handle_errors::Error>;
    async fn delete_address_book(&self, id: i32) -> Result<(), handle_errors::Error>;
    async fn update_address_book(
        &self,
        id: i32,
        address_book: &str,
    ) -> Result<AddressBook, handle_errors::Error>;

    async fn get_all_address_books_lazy(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<AddressBook>, handle_errors::Error>;

    async fn get_address_book_by_id_lazy(
        &self,
        id: i32,
    ) -> Result<Option<AddressBook>, handle_errors::Error>;

    async fn find_address_book_by_name_lazy(
        &self,
        name: &str,
    ) -> Result<Option<AddressBook>, handle_errors::Error>;
}

pub struct AddressBookRepository {
    pool: sqlx::PgPool,
}

impl AddressBookRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IAddressBookRepository for AddressBookRepository {
    async fn get_all_address_books(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<AddressBook>, handle_errors::Error> {
        let q = r#"
            SELECT ab.id AS address_book_id, ab.name AS address_book_name,
            c.id AS contact_id, c.name AS contact_name, c.phone_number c.email
            FROM address_books AS ab
            LEFT JOIN contacts AS c ON ab.id = c.address_book_id LIMIT $1 OFFSET $2
         "#;

        match sqlx::query(q)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| {
                let mut contacts = vec![];
                let address_book_id: AddressBookId = AddressBookId(row.get("address_book_id"));
                let address_book_name: String = row.get("address_book_name");
                let contact_id: ContactId = ContactId(row.get("contact_id"));
                let contact_name: String = row.get("contact_name");
                let email: Option<String> = row.get("email");
                let address = row.get("address");
                let phone_number: Option<String> = row.get("phone_number");
                let contact = Contact {
                    id: contact_id,
                    name: contact_name,
                    address,
                    phone_number,
                    email,
                    address_book_id: address_book_id.clone(),
                };
                contacts.push(contact);

                AddressBook {
                    id: address_book_id,
                    address_book_name,
                    contacts,
                }
            })
            .fetch_all(&self.pool)
            .await
        {
            Ok(address_books) => Ok(address_books),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn get_address_book_by_id(
        &self,
        address_book_id: i32,
    ) -> Result<Option<AddressBook>, handle_errors::Error> {
        let q = "SELECT a.id AS address_book_id, a.address_book_name, c.id AS contact_id, c.name, c.address, c.phone_number, c.email
                    FROM address_books AS a
                    LEFT JOIN contacts AS c ON a.id = c.address_book_id
                    WHERE a.id = $1";
        match sqlx::query(q)
            .bind(address_book_id)
            .map(|row: PgRow| {
                let mut contacts = vec![];
                let address_book_id = AddressBookId(row.get("address_book_id"));
                let address_book_name = row.get("address_book_name");
                let contact_id = ContactId(row.get("contact_id"));
                let name = row.get("name");
                let address = row.get("address");
                let phone_number = row.get("phone_number");
                let email = row.get("email");
                contacts.push(Contact {
                    id: contact_id,
                    name,
                    address,
                    phone_number,
                    email,
                    address_book_id: address_book_id.clone(),
                });
                AddressBook {
                    id: address_book_id,
                    address_book_name,
                    contacts,
                }
            })
            .fetch_optional(&self.pool)
            .await
        {
            Ok(address_book) => Ok(address_book),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn create_address_book(
        &self,
        address_book_name: &str,
    ) -> Result<AddressBook, handle_errors::Error> {
        let q = "INSERT INTO address_books (address_book_name)
                       VALUES ($1) RETURNING id, address_book_name";
        match sqlx::query(q)
            .bind(address_book_name)
            .map(|row: PgRow| AddressBook {
                id: AddressBookId(row.get("id")),
                address_book_name: row.get("address_book_name"),
                contacts: vec![],
            })
            .fetch_one(&self.pool)
            .await
        {
            Ok(address_book) => Ok(address_book),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn find_address_book_by_name(
        &self,
        name: &str,
    ) -> Result<Option<AddressBook>, handle_errors::Error> {
        let q = "SELECT a.id AS address_book_id, a.address_book_name, c.id AS contact_id, c.name, c.address, c.phone_number, c.email
                        FROM address_books AS a
                        LEFT JOIN contacts AS c ON a.id = c.address_book_id
                        WHERE a.address_book_name = $1";
        match sqlx::query(q)
            .bind(name)
            .map(|row: PgRow| {
                let mut contacts = vec![];
                let address_book_id = AddressBookId(row.get("address_book_id"));
                let address_book_name = row.get("address_book_name");
                let contact_id = row.get("contact_id");
                let name = row.get("name");
                let address = row.get("address");
                let phone_number = row.get("phone_number");
                let email = row.get("email");
                let contact = Contact {
                    id: ContactId(contact_id),
                    name,
                    address,
                    phone_number,
                    email,
                    address_book_id: address_book_id.clone(),
                };
                contacts.push(contact);

                AddressBook {
                    id: address_book_id,
                    address_book_name,
                    contacts,
                }
            })
            .fetch_optional(&self.pool)
            .await
        {
            Ok(address_book) => Ok(address_book),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn delete_address_book(&self, id: i32) -> Result<(), handle_errors::Error> {
        let q = "DELETE FROM address_books WHERE id = $1";
        match sqlx::query(q).bind(id).execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn update_address_book(
        &self,
        id: i32,
        address_book_name: &str,
    ) -> Result<AddressBook, handle_errors::Error> {
        let q = "UPDATE address_books SET address_book_name = $1 WHERE id = $2 RETURNING id, address_book_name";
        match sqlx::query(q)
            .bind(address_book_name)
            .bind(id)
            .map(|row: PgRow| AddressBook {
                id: AddressBookId(row.get("id")),
                address_book_name: row.get("address_book_name"),
                contacts: vec![],
            })
            .fetch_one(&self.pool)
            .await
        {
            Ok(address_book) => Ok(address_book),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }
    async fn get_address_book_by_id_lazy(
        &self,
        id: i32,
    ) -> Result<Option<AddressBook>, handle_errors::Error> {
        let q = "SELECT * FROM address_books WHERE id = $1";
        match sqlx::query(q)
            .bind(id)
            .map(|row: PgRow| AddressBook {
                id: AddressBookId(row.get("id")),
                address_book_name: row.get("address_book_name"),
                contacts: vec![],
            })
            .fetch_optional(&self.pool)
            .await
        {
            Ok(address_book) => Ok(address_book),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn find_address_book_by_name_lazy(
        &self,
        name: &str,
    ) -> Result<Option<AddressBook>, handle_errors::Error> {
        let q = "SELECT * FROM address_books WHERE address_book_name = $1";
        match sqlx::query(q)
            .bind(name)
            .map(|row: PgRow| AddressBook {
                id: AddressBookId(row.get("id")),
                address_book_name: row.get("address_book_name"),
                contacts: vec![],
            })
            .fetch_optional(&self.pool)
            .await
        {
            Ok(address_book) => Ok(address_book),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }

    async fn get_all_address_books_lazy(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<AddressBook>, handle_errors::Error> {
        let q = "SELECT * FROM address_books LIMIT $1 OFFSET $2";
        match sqlx::query(q)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| AddressBook {
                id: AddressBookId(row.get("id")),
                address_book_name: row.get("address_book_name"),
                contacts: vec![],
            })
            .fetch_all(&self.pool)
            .await
        {
            Ok(address_books) => Ok(address_books),
            Err(e) => Err(handle_errors::Error::DatabaseQueryError(e)),
        }
    }
}
