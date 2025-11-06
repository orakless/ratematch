pub struct PageParameters {
    page: i64,
    per_page: i64,
}

impl PageParameters {
    pub fn page(&self) -> i64 {
        self.page
    }

    pub fn per_page(&self) -> i64 {
        self.per_page
    }
}
