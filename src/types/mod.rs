pub mod address_book;
pub mod contact;
pub mod pagination;

#[derive(serde::Deserialize)]
pub enum LoadingStrategy {
    #[serde(rename = "lazy")]
    Lazy,
    #[serde(rename = "eager")]
    Eager,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub limit: i32,
    pub offset: i32,
    pub loading_strategy: Option<LoadingStrategy>,
}
