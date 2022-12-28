use std::net::IpAddr;
use std::num::TryFromIntError;
use std::time::Duration;

use async_graphql::Object;
use log::warn;

use crate::api::location::Location;
use crate::error::BackendError;
use crate::topology::model::device::DeviceRef;
use crate::topology::query::get_topology;

#[derive(Debug)]
pub struct Device(DeviceRef);

impl Device {
    pub fn new(d: DeviceRef) -> Self {
        Device(d)
    }
}

impl From<DeviceRef> for Device {
    fn from(d: DeviceRef) -> Self {
        Device(d)
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
        self.0.get_id()
    }
    async fn name(&self) -> &str {
        self.0.get_name()
    }
    async fn ping(&self) -> Result<PingResult, BackendError> {
        let ip_addr: IpAddr = self
            .0
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
        self.0.location().map(Location::new)
    }
    async fn has_routeros(&self) -> bool {
        self.0.has_routeros()
    }
}

pub async fn get_device(id: u32) -> Result<Option<Device>, BackendError> {
    let topology = get_topology().await?;
    Ok(topology.get_device_by_id(id).map(Device::new))
}

pub async fn list_devices() -> Result<Vec<Device>, BackendError> {
    let topology = get_topology().await?;
    let results = topology.list_devices_map(|d| {
        if d.has_routeros() {
            Some(Device::new(d))
        } else {
            None
        }
    });
    Ok(results)
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
