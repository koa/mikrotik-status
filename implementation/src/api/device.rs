use std::collections::HashSet;
use std::net::IpAddr;

use async_graphql::futures_util::future::join_all;
use async_graphql::Object;
use log::{error, info};

use ipnet::IpNet;

use crate::error::BackendError;
use crate::netbox;
use crate::netbox::graphql::list_devices::{get_device, list_devices, GetDevice, ListDevices};

#[derive(Debug)]
pub struct Device {
    response: get_device::GetDeviceDevice,
}

impl Device {
    pub fn new(data: get_device::GetDeviceDevice) -> Device {
        Device { response: data }
    }
}

#[Object]
impl Device {
    async fn id(&self) -> i32 {
        self.response
            .id
            .parse()
            .expect("Cannot parse id of fetched device")
    }
    async fn name(&self) -> &str {
        self.response
            .name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    }
    async fn ping(&self) -> Result<bool, BackendError> {
        let ip_addr: IpAddr = if let Some(ipv4) = self.response.primary_ip4.as_ref() {
            Ok(ipv4.address.parse::<IpNet>().unwrap().addr())
        } else if let Some(ipv6) = self.response.primary_ip6.as_ref() {
            Ok(ipv6.address.parse::<IpNet>().unwrap().addr())
        } else {
            Err(BackendError::MissingIpAddress())
        }?;

        info!("Send ping to {ip_addr}");
        let ping_result = surge_ping::ping(ip_addr, &[0; 256]).await;
        Ok(match ping_result {
            Ok((data, duration)) => {
                info!("Success: {data:?}, {duration:?}");
                true
            }
            Err(e) => {
                error!("Error from ping: {e:?}");
                false
            }
        })
    }
}

pub async fn get_device(id: i64) -> Result<Option<Device>, BackendError> {
    let found_device = netbox::query::<GetDevice>(get_device::Variables { id }).await?;
    Ok(found_device.device.map(|d| Device::new(d)))
}

pub async fn list_devices() -> Result<Vec<Device>, BackendError> {
    let device_list = netbox::query::<ListDevices>(list_devices::Variables {}).await?;
    let routeros_device_types: HashSet<_> = device_list
        .device_type_list
        .iter()
        .flatten()
        .filter(|type_entry| {
            type_entry
                .tags
                .iter()
                .flatten()
                .flatten()
                .find(|option_tag| option_tag.slug == "routeros")
                .is_some()
        })
        .map(|t| t.id.as_str())
        .collect();

    let answers = join_all(
        device_list
            .device_list
            .iter()
            .flatten()
            .filter(|dev| routeros_device_types.contains(dev.device_type.id.as_str()))
            .map(|dev| {
                let id = dev.id.parse()?;
                Ok(netbox::query::<GetDevice>(get_device::Variables { id }))
            })
            .map(|r| async {
                match r {
                    Ok(value) => value.await,
                    Err(e) => Err(e),
                }
            }),
    )
    .await;
    let mut errors = Vec::new();
    let mut results = Vec::with_capacity(answers.len());
    for answer in answers {
        match answer {
            Ok(result) => results.push(Device::new(result.device.expect("taken empty response"))),
            Err(e) => errors.push(e),
        }
    }
    if !errors.is_empty() {
        return if errors.len() == 1 {
            Err(errors.remove(0))
        } else {
            Err(BackendError::Umbrella(errors))
        };
    }
    Ok(results)
}
