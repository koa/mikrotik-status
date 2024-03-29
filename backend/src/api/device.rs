use std::net::IpAddr;
use std::sync::Arc;
use std::{num::TryFromIntError, time::Duration};

use async_graphql::{Enum, Object};
use ipnet::IpNet;
use log::warn;

use crate::api::device_type::DeviceType;
use crate::api::location::Location;
use crate::topology::model;
use crate::topology::model::Topology;
use crate::{error::BackendError, topology::query::get_topology};

#[derive(Debug)]
pub struct Device {
    device: Arc<model::Device>,
    topology: Arc<Topology>,
}

#[derive(Debug)]
pub struct DevicePort(Arc<model::DevicePort>);

#[derive(Debug)]
pub struct IpNetApi(IpNet);

impl IpNetApi {
    fn new(value: IpNet) -> Self {
        Self(value)
    }
}

impl Device {
    pub fn new(device: Arc<model::Device>, topology: Arc<Topology>) -> Self {
        Device { device, topology }
    }
}

impl From<Arc<model::DevicePort>> for DevicePort {
    fn from(value: Arc<model::DevicePort>) -> Self {
        DevicePort(value)
    }
}

pub struct PingResult {
    answer: Option<PingAnswer>,
}

pub struct PingAnswer {
    duration: Duration,
}

#[Object]
impl Device {
    async fn id(&self) -> u32 {
        self.device.id()
    }
    async fn name(&self) -> &str {
        self.device.name()
    }
    async fn ping(&self) -> Result<PingResult, BackendError> {
        let ip_addr: IpAddr = self
            .device
            .get_loopback_address()
            .ok_or(BackendError::MissingIpAddress())?;

        //info!("Send ping to {ip_addr}");
        let ping_result = surge_ping::ping(ip_addr, &[0; 256]).await;
        Ok(match ping_result {
            Ok((_data, duration)) => {
                //      info!("Success: {data:?}, {duration:?}");
                PingResult {
                    answer: Some(PingAnswer { duration }),
                }
            }
            Err(e) => {
                warn!("Error from ping: {e:?}");
                PingResult { answer: None }
            }
        })
    }
    async fn location(&self) -> Option<Location> {
        self.device
            .location()
            .and_then(|id| self.topology.get_location(id))
            .map(|l| Location::new(l, self.topology.clone()))
    }

    async fn device_type(&self) -> Option<DeviceType> {
        let type_idx = self.device.device_type();
        self.topology
            .get_device_type(type_idx)
            .map(|tid| DeviceType::new(tid, self.topology.clone()))
    }

    async fn ports(&self) -> Vec<DevicePort> {
        self.device
            .ports()
            .into_iter()
            .map(DevicePort::from)
            .collect()
    }
}

#[Object]
impl DevicePort {
    async fn name(&self) -> &str {
        self.0.get_name()
    }
    async fn address(&self, address_type: Option<IpFamily>) -> Vec<IpNetApi> {
        self.0
            .list_nets()
            .into_iter()
            .map(IpNetApi::new)
            .filter(|net| {
                address_type
                    .map(|t| net.inner_family() == t)
                    .unwrap_or(true)
            })
            .collect()
    }
}

pub async fn get_device(id: u32) -> Result<Option<Device>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology
        .get_device_by_id(id)
        .map(|d| Device::new(d, topology)))
}

pub async fn list_devices() -> Result<Vec<Device>, BackendError> {
    let topology = get_topology().await?;
    let results = topology.list_devices_map(|d| {
        if d.has_routeros() {
            Some(Device::new(d.clone(), topology.clone()))
        } else {
            None
        }
    });
    Ok(results)
}

impl IpNetApi {
    fn inner_family(&self) -> IpFamily {
        match self.0 {
            IpNet::V4(_) => IpFamily::V4,
            IpNet::V6(_) => IpFamily::V6,
        }
    }
}

#[Object]
impl IpNetApi {
    /// format the whole network as string
    async fn as_string(&self) -> String {
        self.0.to_string()
    }
    /// determine ip address family
    async fn family(&self) -> IpFamily {
        self.inner_family()
    }
}
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum IpFamily {
    V4,
    V6,
}

#[Object]
impl PingAnswer {
    async fn duration_in_ms(&self) -> Result<u64, TryFromIntError> {
        self.duration.as_millis().try_into()
    }
}
#[Object]
impl PingResult {
    async fn answer(&self) -> &Option<PingAnswer> {
        &self.answer
    }
}
