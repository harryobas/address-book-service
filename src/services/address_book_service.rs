use crate::repositories::address_book_repo::{AddressBookRepository, IAddressBookRepository};
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
        offset: i32,
        load_strategy: &str,
    ) -> Result<Vec<AddressBook>, handle_errors::Error> {
        match load_strategy {
            "lazy" => self.repo.get_all_address_books_lazy(limit, offset).await,
            "eager" => self.repo.get_all_address_books(limit, offset).await,
            _ => Err(handle_errors::Error::InvalidLoadStrategy),
        }
    }

    pub async fn get_address_book_by_id(
        &self,
        id: i32,
        load_strategy: &str,
    ) -> Result<AddressBook, handle_errors::Error> {
        match load_strategy {
            "lazy" => self.repo.get_address_book_by_id_lazy(id).await,
            "eager" => self.repo.get_address_book_by_id(id).await,
            _ => Err(handle_errors::Error::InvalidLoadStrategy),
        }
    }
    pub async fn get_address_book_by_name(
        &self,
        address_book_name: &str,
        load_strategy: &str,
    ) -> Result<AddressBook, handle_errors::Error> {
        match load_strategy {
            "lazy" => {
                self.repo
                    .find_address_book_by_name_lazy(address_book_name)
                    .await
            }
            "eager" => self.repo.find_address_book_by_name(address_book_name).await,
            _ => Err(handle_errors::Error::InvalidLoadStrategy),
        }
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
