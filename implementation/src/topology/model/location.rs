#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Location {
    id: u32,
    name: String,
}

impl Location {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct LocationIdx {
    site_idx: usize,
    location_idx: usize,
}

impl LocationIdx {
    pub fn new(site_idx: usize, location_idx: usize) -> Self {
        Self {
            site_idx,
            location_idx,
        }
    }

    pub fn site_idx(&self) -> usize {
        self.site_idx
    }
    pub fn location_idx(&self) -> usize {
        self.location_idx
    }
}
