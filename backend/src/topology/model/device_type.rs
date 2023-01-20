#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct DeviceType {
    name: String,
    id: u32,
    has_routeros: bool,
}

impl DeviceType {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn new(name: String, id: u32, has_routeros: bool) -> Self {
        Self {
            name,
            id,
            has_routeros,
        }
    }

    pub fn has_routeros(&self) -> bool {
        self.has_routeros
    }
}
