use anyhow::{anyhow, Result, Ok};
use bytes::Bytes;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct POIData {
    pub id: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
    pub street: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub meter_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PowerData {
    pub id: Option<String>,
    pub timestamp: Option<Date>,
    pub power: Option<f64>,
} 