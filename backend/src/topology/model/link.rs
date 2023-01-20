use crate::topology::model::device::{DeviceBuilder, PortIdx};
use crate::topology::model::TopologyError;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Link {
    path: Vec<LinkSegment>,
}

impl Link {
    pub fn path(&self) -> &Vec<LinkSegment> {
        &self.path
    }
}

pub struct LinkBuilder {
    path: Vec<LinkSegment>,
}

impl LinkBuilder {
    pub fn append_segment(
        &mut self,
        devices: &[DeviceBuilder],
        left_device_idx: usize,
        left_port: usize,
        right_device_idx: usize,
        right_port: usize,
    ) -> Result<usize, TopologyError> {
        let left_device = devices
            .get(left_device_idx)
            .ok_or(TopologyError::MissingDeviceReference(left_device_idx))?;
        if left_device.ports().len() <= left_port {
            return Err(TopologyError::MissingPortReference {
                device_idx: left_device_idx,
                port_idx: left_device_idx,
            });
        }
        let right_device = devices
            .get(right_device_idx)
            .ok_or(TopologyError::MissingDeviceReference(right_device_idx))?;
        if right_device.ports().len() <= right_port {
            return Err(TopologyError::MissingPortReference {
                device_idx: right_device_idx,
                port_idx: right_device_idx,
            });
        }
        if let Some(last_segment) = self.path.last() {
            let last_device = last_segment.right_port.device_idx();
            if last_device != left_device_idx {
                return Err(TopologyError::InvalidPath {
                    last_device,
                    current_device: left_device_idx,
                });
            }
        }
        self.path.push(LinkSegment {
            left_port: PortIdx::new(left_device_idx, left_port),
            right_port: PortIdx::new(right_device_idx, right_port),
        });
        Ok(self.path.len() - 1)
    }
    pub fn build(self) -> Link {
        Link { path: self.path }
    }
    pub fn new() -> Self {
        Self { path: vec![] }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct LinkSegment {
    left_port: PortIdx,
    right_port: PortIdx,
}

impl LinkSegment {
    pub fn ports(&self) -> [&PortIdx; 2] {
        [&self.left_port, &self.right_port]
    }

    pub fn left_port(&self) -> PortIdx {
        self.left_port
    }
    pub fn right_port(&self) -> PortIdx {
        self.right_port
    }
}
