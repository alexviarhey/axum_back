use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryPagination {
    page: usize,
    size: usize,
}

impl QueryPagination {
    pub fn skip(&self) -> usize {
        self.page * self.size
    }

    pub fn limit(&self) -> usize {
        self.size
    }
}

impl Default for QueryPagination {
    fn default() -> Self {
        Self { page: 0, size: 10 }
    }
}
