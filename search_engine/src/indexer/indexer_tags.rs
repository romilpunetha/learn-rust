pub struct IndexerTags {
    title: u32,
    body: u32,
    info_box: u32,
    category: u32,
}

impl IndexerTags {
    pub fn new() -> Self {
        IndexerTags {
            title: 0,
            body: 0,
            info_box: 0,
            category: 0,
        }
    }

    pub fn get_title(&self) -> u32 { self.title }

    pub fn get_body(&self) -> u32 { self.body }

    pub fn get_info_box(&self) -> u32 { self.info_box }

    pub fn get_category(&self) -> u32 { self.category }

    pub fn increment_title(&mut self) {
        self.title += 1;
    }

    pub fn increment_body(&mut self) { self.body += 1; }

    pub fn increment_info_box(&mut self) {
        self.info_box += 1;
    }

    pub fn increment_category(&mut self) {
        self.category += 1;
    }
}