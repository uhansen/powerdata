use anyhow::{anyhow, Result, Ok};
use bytes::Bytes;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use crate::utils::get_last_param_from_route;

fn as_param<'a>(value: &'a Option<String>) -> Option<ParameterValue<'a>> {
    match value {
        Some(value) => Some(ParameterValue::Str(value.as_str())),
        None => None
    }
}

fn as_nullable_param<'a>(value: &'a Option<String>) -> ParameterValue<'a> {
    match as_param(value) {
        Some(value) => value,
        None => ParameterValue::DbNull,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AddressData {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CustomerData {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub key: String,
    pub address: AddressData,
    pub meter_number: Option<String>
}

impl CustomerData {
    pub fn new(
        id: Option<String>,
        first_name: String,
        last_name: String,
        email: String,
        key: String,
        address: AddressData,
        meter_number: Option<String>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            key,
            address,
            meter_number,
        }

    }

    pub(crate) fn insert(&self) -> Result<()> {
        let params = vec![
            as_param(&self.id).ok_or(anyhow!("The id field is currently required for insert"))?,
            ParameterValue::Str(&self.first_name),
            ParameterValue::Str(&self.last_name),
            ParameterValue::Str(&self.email),
            ParameterValue::Str(&self.key),
            ParameterValue::Str(&self.address.street),
            ParameterValue::Str(&self.address.city),
            ParameterValue::Str(&self.address.zip_code),
            ParameterValue::Str(&self.address.country),
            ParameterValue::Float(self.address.longitude),
            ParameterValue::Float(self.address.latitude),
            as_param(&self.meter_number).ok_or(anyhow!("The meter_number field is currently required for insert"))?,
        ];
        sqlite::execute("INSERT INTO Customer (id, first_name, last_name, email, key, street, city, zip, country, longitude, latitude, meterno) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", &params)?;
        Ok(())
    }
}