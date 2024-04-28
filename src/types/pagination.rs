use std::collections::HashMap;
use handle_errors::Error;

#[derive(Clone, Debug, Default)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: u32
}

impl Pagination {
    pub fn extract_pagination(params: HashMap<String, String>) -> Result<Self, Error> {
        if params.contains_key("limit") && params.contains_key("offset") {
            let limit = params.get("limit")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?;
            let offset = params.get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?;
            Ok(Self { limit: Some(limit), offset })
        } else {
            Err(Error::MissingParameters)

        }
    }
}