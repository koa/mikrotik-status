#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Location {
    id: u32,
    name: String,
    site: Option<usize>,
    devices: Vec<usize>,
}

pub struct LocationBuilder {
    id: u32,
    name: String,
    site: Option<usize>,
    devices: Vec<usize>,
}

impl LocationBuilder {
    pub fn site(&mut self, site_idx: usize) -> &mut Self {
        self.site = Some(site_idx);
        self
    }
    pub fn build(mut self) -> Location {
        self.devices.shrink_to_fit();
        Location {
            id: self.id,
            name: self.name,
            site: self.site,
            devices: self.devices,
        }
    }
}

impl Location {
    pub fn builder(id: u32, name: String, devices: Vec<usize>) -> LocationBuilder {
        LocationBuilder {
            id,
            name,
            site: None,
            devices,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn site(&self) -> Option<usize> {
        self.site
    }

    pub fn devices(&self) -> &Vec<usize> {
        &self.devices
    }
}
