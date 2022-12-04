use std::fmt::Formatter;
use std::ops::Deref;

use graphql_client::GraphQLQuery;
use log::info;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Decimal(f64);

impl From<Decimal> for f64 {
    fn from(value: Decimal) -> Self {
        value.0
    }
}

impl From<f64> for Decimal {
    fn from(v: f64) -> Self {
        Decimal(v)
    }
}

struct DecimalVisitor;

impl<'de> Visitor<'de> for DecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a float value encoded as string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Decimal(v.parse().map_err(|e| {
            E::custom(format!("Cannot parse decimal: {v}: {e}"))
        })?))
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DecimalVisitor)
    }
}

impl Deref for Decimal {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema/netbox.graphql",
    query_path = "src/topology/list_devices.graphql",
    response_derives = "Debug",
    variables_derives = "Default,Debug"
)]
pub struct ListAllDevices;
