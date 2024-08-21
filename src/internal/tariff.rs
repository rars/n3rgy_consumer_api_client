use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{ElectricityTariff, GasTariff, StandingCharge, TimestampedValue};

use super::utils::parse_n3rgy_timestamp;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_snake_case)]
pub(crate) struct StandingChargeDto {
    startDate: String,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct TimestampedValueDto {
    timestamp: String,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_snake_case)]
pub(crate) struct ElectricityTariffDto {
    standingCharges: Vec<StandingChargeDto>,
    prices: Vec<TimestampedValueDto>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_snake_case)]
pub(crate) struct GasTariffDto {
    standingCharges: Vec<StandingChargeDto>,
    prices: Vec<TimestampedValueDto>,
}

impl TryFrom<&ElectricityTariffDto> for ElectricityTariff {
    type Error = chrono::ParseError;

    fn try_from(item: &ElectricityTariffDto) -> Result<Self, Self::Error> {
        Ok(ElectricityTariff {
            standing_charges: item
                .standingCharges
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<_, _>>()?,
            prices: item
                .prices
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<&GasTariffDto> for GasTariff {
    type Error = chrono::ParseError;

    fn try_from(item: &GasTariffDto) -> Result<Self, Self::Error> {
        Ok(GasTariff {
            standing_charges: item
                .standingCharges
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<_, _>>()?,
            prices: item
                .prices
                .iter()
                .map(|x| x.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<&StandingChargeDto> for StandingCharge {
    type Error = chrono::ParseError;

    fn try_from(item: &StandingChargeDto) -> Result<Self, Self::Error> {
        Ok(StandingCharge {
            start_date: NaiveDate::parse_from_str(&item.startDate, "%Y-%m-%d")?,
            value: item.value,
        })
    }
}

impl TryFrom<&TimestampedValueDto> for TimestampedValue {
    type Error = chrono::ParseError;

    fn try_from(item: &TimestampedValueDto) -> Result<Self, Self::Error> {
        Ok(TimestampedValue {
            timestamp: parse_n3rgy_timestamp(&item.timestamp)?,
            value: item.value,
        })
    }
}
