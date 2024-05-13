use crate::repositories::address_book_repo::IAddressBookRepository;
use crate::types::address_book::{AddressBook, NewAddressBook};
pub struct AddressBookService {}

impl AddressBookService {
    pub async fn get_all_address_books<T: IAddressBookRepository>(
        repo: T,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<AddressBook>, handle_errors::Error> {
        repo.get_all_address_books(limit, offset).await
    }

    pub async fn get_address_book_by_id<T: IAddressBookRepository>(
        repo: T,
        id: i32,
    ) -> Result<AddressBook, handle_errors::Error> {
        repo.get_address_book_by_id(id).await
    }
    pub async fn get_address_book_by_name<T: IAddressBookRepository>(
        repo: T,
        address_book_name: String,
    ) -> Result<AddressBook, handle_errors::Error> {
        repo.find_address_book_by_name(address_book_name).await
    }

    pub async fn add_address_book<T: IAddressBookRepository>(
        repo: T,
        address_book: NewAddressBook,
    ) -> Result<AddressBook, handle_errors::Error> {
        repo.create_address_book(address_book.address_book_name)
            .await
    }

    pub async fn delete_address_book<T: IAddressBookRepository>(
        repo: T,
        id: i32,
    ) -> Result<(), handle_errors::Error> {
        repo.delete_address_book(id).await
    }

    pub async fn update_address_book<T: IAddressBookRepository>(
        repo: T,
        id: i32,
        address_book: NewAddressBook,
    ) -> Result<AddressBook, handle_errors::Error> {
        repo.update_address_book(id, &address_book.address_book_name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::address_book_repo::MockIAddressBookRepository;
    use crate::types::address_book::{AddressBook, AddressBookId};
    use mockall::predicate::eq;

    fn create_repo() -> MockIAddressBookRepository {
        MockIAddressBookRepository::new()
    }

    fn create_address_book() -> AddressBook{
        AddressBook {
            id: AddressBookId(1),
            address_book_name: String::from("address_book_1"),
            contacts: vec![],
        }

    }

    #[tokio::test]
    async fn test_get_all_address_books() {
        let mut repo = create_repo();

        let address_books = vec![
            AddressBook {
                id: AddressBookId(1),
                address_book_name: String::from("address_book_1"),
                contacts: vec![],
            },
            AddressBook {
                id: AddressBookId(2),
                address_book_name: String::from("address_book_2"),
                contacts: vec![],
            },
        ];
        let limit = Some(2);
        let offset = 0;
        repo.expect_get_all_address_books()
            .with(eq(limit), eq(offset))
            .once()
            .returning(move |_, _| {
                let address_books = address_books.clone();
                Box::pin(async move { Ok(address_books) })
            });

        let result = AddressBookService::get_all_address_books(repo, limit, offset).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_address_book_by_id() {
        let mut repo = create_repo();
        let address_book = create_address_book();
        let id = 1;

        repo.expect_get_address_book_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| {
                let address_book = address_book.clone();
                Box::pin(async move { Ok(address_book) })
                
            });

        let result = AddressBookService::get_address_book_by_id(repo, id).await;
        assert!(result.is_ok());

    }
    #[tokio::test]
    async fn test_get_address_book_by_name() {
        let address_book = create_address_book();
        let address_book_name = String::from("address_book 1");
        let mut repo = create_repo();

        repo.expect_find_address_book_by_name()
            .with(eq(address_book_name.clone()))
            .once()
            .returning(move |_| {
                let address_book = address_book.clone();
                Box::pin(async move {Ok(address_book)})
            });

        let result = AddressBookService::get_address_book_by_name(repo, address_book_name).await;
        assert!(result.is_ok());

    }
    #[tokio::test]
    async fn test_add_address_book() {
        let new_address_book = NewAddressBook {
            address_book_name: String::from("address_book_1"),
        };
        let address_book = create_address_book();
        let mut repo = create_repo();

        repo.expect_create_address_book()
           .with(eq(new_address_book.address_book_name.clone()))
           .once()
           .returning(move |_| {
                let address_book = address_book.clone();
                Box::pin(async move { Ok(address_book) })
            });

            let result = AddressBookService::add_address_book(repo, new_address_book).await;
            assert!(result.is_ok());

    }

}
