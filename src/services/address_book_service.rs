use crate::repositories::address_book_repo::IAddressBookRepository;
use crate::types::address_book::{AddressBook, NewAddressBook};
pub struct AddressBookService<T: IAddressBookRepository> {
    pub repo: T,
}

impl<T: IAddressBookRepository> AddressBookService<T> {
    pub fn new(repo: T) -> AddressBookService<T> {
        AddressBookService { repo }
    }
    pub async fn get_all_address_books(
        &self,
        limit: Option<i32>,
        offset: i32
    ) -> Result<Vec<AddressBook>, handle_errors::Error> {
        self.repo
            .get_all_address_books(limit, offset)
            .await

    }

    pub async fn get_address_book_by_id(
        &self,
        id: i32,
    ) -> Result<AddressBook, handle_errors::Error> {
        self.repo
            .get_address_book_by_id(id)
            .await

    }
    pub async fn get_address_book_by_name(
        &self,
        address_book_name: &str,
        
    ) -> Result<AddressBook, handle_errors::Error> {
        self.repo
            .find_address_book_by_name(&address_book_name)
            .await
        
    }

    pub async fn add_address_book(
        &self,
        address_book: NewAddressBook,
    ) -> Result<AddressBook, handle_errors::Error> {
        self.repo
            .create_address_book(&address_book.address_book_name)
            .await
    }

    pub async fn delete_address_book(&self, id: i32) -> Result<(), handle_errors::Error> {
        self.repo.delete_address_book(id).await
    }

    pub async fn update_address_book(
        &self,
        id: i32,
        address_book: NewAddressBook,
    ) -> Result<AddressBook, handle_errors::Error> {
        self.repo
            .update_address_book(id, &address_book.address_book_name)
            .await
    }
}
