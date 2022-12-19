use std::sync::Arc;

use crate::topology::model::device::{PortIdx, PortSide};
use crate::topology::model::{Topology, TopologyBuilder, TopologyError};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Link {
    path: Vec<LinkSegment>,
}

impl Link {
    pub fn path(&self) -> &Vec<LinkSegment> {
        &self.path
    }
}

pub struct LinkBuilder<'a> {
    topo_builder: &'a mut TopologyBuilder,
    path: Vec<LinkSegment>,
}

impl<'a> LinkBuilder<'a> {
    pub fn append_segment(
        &mut self,
        left_device_idx: usize,
        left_port: usize,
        right_device_idx: usize,
        right_port: usize,
    ) -> Result<usize, TopologyError> {
        let left_device = self
            .topo_builder
            .devices
            .get(left_device_idx)
            .ok_or(TopologyError::MissingDeviceReference(left_device_idx))?;
        if left_device.ports().len() <= left_port {
            return Err(TopologyError::MissingPortReference {
                device_idx: left_device_idx,
                port_idx: left_device_idx,
            });
        }
        let right_device = self
            .topo_builder
            .devices
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
    pub fn build(self) -> usize {
        self.topo_builder.links.push(Link { path: self.path });
        self.topo_builder.links.len() - 1
    }
    pub fn new(topo_builder: &'a mut TopologyBuilder) -> Self {
        Self {
            topo_builder,
            path: vec![],
        }
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

pub struct LinkPortRef {
    topology: Arc<Topology>,
    link: Arc<Link>,
    hit_idx: Vec<(PortSide, usize)>,
}

impl LinkPortRef {
    pub fn new(topology: Arc<Topology>, link: Arc<Link>, hit_idx: Vec<(PortSide, usize)>) -> Self {
        Self {
            topology,
            link,
            hit_idx,
        }
    }

    pub fn topology(&self) -> &Arc<Topology> {
        &self.topology
    }
    pub fn link(&self) -> &Arc<Link> {
        &self.link
    }
    pub fn hit_idx(&self) -> &Vec<(PortSide, usize)> {
        &self.hit_idx
    }
}
