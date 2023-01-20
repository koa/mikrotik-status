use std::collections::HashSet;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Site {
    id: u32,
    name: String,
    address: String,
    locations: Vec<usize>,
}

impl Site {
    pub(crate) fn new(id: u32, name: String, address: String, locations: HashSet<usize>) -> Site {
        Site {
            id,
            name,
            address,
            locations: locations.into_iter().collect(),
        }
    }
    pub fn builder(id: u32, name: String, address: String) -> SiteBuilder {
        SiteBuilder {
            id,
            name,
            address,
            locations: vec![],
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn locations(&self) -> &Vec<usize> {
        &self.locations
    }
}

pub struct SiteBuilder {
    id: u32,
    name: String,
    address: String,
    locations: Vec<(u32, String)>,
}

impl SiteBuilder {
    pub fn new(id: u32, name: String, address: String) -> Self {
        Self {
            id,
            name,
            address,
            locations: vec![],
        }
    }
    pub(crate) fn append_location(&mut self, id: u32, name: String) {
        self.locations.push((id, name));
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn destruct(self) -> (u32, String, String) {
        (self.id, self.name, self.address)
    }
    pub fn locations(&self) -> &Vec<(u32, String)> {
        &self.locations
    }
}
