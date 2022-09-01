use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryPagination {
    page: i64,
    size: i64,
}

impl QueryPagination {
    pub fn skip(&self) -> Option<u64> {
        Some((self.page * self.size) as u64)
    }

    pub fn limit(&self) -> Option<i64> {
        Some(self.size)
    }
}

impl Default for QueryPagination {
    fn default() -> Self {
        Self { page: 0, size: 10 }
    }
}
