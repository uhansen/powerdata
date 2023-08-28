use anyhow::{anyhow, Result, Ok};
use bytes::Bytes;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

pub(crate) struct AdressData {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CustomerData {
    pub id: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
    pub key: String,
    pub address: AdressData,
    pub meter_number: Option<String>
}