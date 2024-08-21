use serde::{Deserialize, Serialize};

use crate::{ElectricityConsumption, GasConsumption};

use super::utils::parse_n3rgy_timestamp;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct ElectricityConsumptionDto {
    pub timestamp: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct GasConsumptionDto {
    pub timestamp: String,
    pub value: f64,
}

impl TryFrom<&ElectricityConsumptionDto> for ElectricityConsumption {
    type Error = chrono::ParseError;

    fn try_from(item: &ElectricityConsumptionDto) -> Result<Self, Self::Error> {
        Ok(ElectricityConsumption {
            timestamp: parse_n3rgy_timestamp(&item.timestamp)?,
            value: item.value,
        })
    }
}

impl TryFrom<&GasConsumptionDto> for GasConsumption {
    type Error = chrono::ParseError;

    fn try_from(item: &GasConsumptionDto) -> Result<Self, Self::Error> {
        Ok(GasConsumption {
            timestamp: parse_n3rgy_timestamp(&item.timestamp)?,
            value: item.value,
        })
    }
}
