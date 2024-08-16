use thiserror::Error;

use chrono::{NaiveDate, NaiveDateTime, ParseResult};
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use url::ParseError;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

const BASE_URL: &str = "https://consumer-api.data.n3rgy.com";

fn parse_n3rgy_timestamp(timestamp: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M")
}

pub enum EnergyType {
    Electricity,
    Gas,
}

pub enum ReadingType {
    Consumption,
    Tariff,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ElectricityConsumptionDto {
    timestamp: String,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct StandingChargeDto {
    startDate: String,
    value: f64,
}

pub struct StandingCharge {
    start_date: NaiveDate,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TimestampedValueDto {
    timestamp: String,
    value: f64,
}

pub struct TimestampedValue {
    timestamp: NaiveDateTime,
    value: f64,
}

pub struct ElectricityTariff {
    standing_charges: Vec<StandingCharge>,
    prices: Vec<TimestampedValue>,
}

pub struct GasTariff {
    standing_charges: Vec<StandingCharge>,
    prices: Vec<TimestampedValue>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct ElectricityTariffDto {
    standingCharges: Vec<StandingChargeDto>,
    prices: Vec<TimestampedValueDto>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct GasTariffDto {
    standingCharges: Vec<StandingChargeDto>,
    prices: Vec<TimestampedValueDto>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GasConsumptionDto {
    timestamp: String,
    value: f64,
}

pub struct ElectricityConsumption {
    timestamp: NaiveDateTime,
    value: f64,
}

pub struct GasConsumption {
    timestamp: NaiveDateTime,
    value: f64,
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

impl TryFrom<&ElectricityTariffDto> for ElectricityTariff {
    type Error = chrono::ParseError;

    fn try_from(item: &ElectricityTariffDto) -> Result<Self, Self::Error> {
        Ok(ElectricityTariff {
            standing_charges: item.standingCharges.iter().map(|x| x.try_into()).collect::<Result<_, _>>()?,
            prices: item.prices.iter().map(|x| x.try_into()).collect::<Result<_, _>>()?,
        })
    }
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

pub async fn get_electricity_consumption(
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<ElectricityConsumption>, GetRecordsError> {
    let dto_records = get_records::<ElectricityConsumptionDto>(
        EnergyType::Electricity,
        ReadingType::Consumption,
        start,
        end,
    )
    .await?;

    let records: Result<Vec<ElectricityConsumption>, chrono::ParseError> =
        dto_records.iter().map(|x| x.try_into()).collect();

    Ok(records?)
}

pub async fn get_electricity_tariff(start: NaiveDate, end: NaiveDate) -> Result<Vec<ElectricityTariff>,  {
    let dto_records = get_records::<ElectricityTariffDto>(
        EnergyType::Electricity,
        ReadingType::Tariff,
        start,
        end,
    )
    .await?;

    let records: Result<Vec<ElectricityTariff>, chrono::ParseError> =
        dto_records.iter().map(|x| x.try_into()).collect();

    Ok(records?)
}

pub async fn get_gas_consumption(
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<GasConsumption>, GetRecordsError> {
    let dto_records =
        get_records::<GasConsumptionDto>(EnergyType::Gas, ReadingType::Consumption, start, end)
            .await?;

    let records: Result<Vec<GasConsumption>, chrono::ParseError> =
        dto_records.iter().map(|x| x.try_into()).collect();

    Ok(records?)
}

pub async fn get_gas_tariff(start: NaiveDate, end: NaiveDate) -> Result<Vec<GasTariff>, GetRecordsError> {
    let dto_records = get_records::<GasTariffDto>(
        EnergyType::Electricity,
        ReadingType::Tariff,
        start,
        end,
    )
    .await?;

    let records: Result<Vec<GasTariff>, chrono::ParseError> =
        dto_records.iter().map(|x| x.try_into()).collect();

    Ok(records?)
}

#[derive(Debug, Error)]
pub enum GetRecordsError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("Chrono parser error: {0}")]
    ChronoParse(#[from] chrono::ParseError),
}

pub async fn get_records<T>(
    energy_type: EnergyType,
    reading_type: ReadingType,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<T>, GetRecordsError>
where
    T: DeserializeOwned,
{
    let energy_type_str = match energy_type {
        EnergyType::Electricity => "electricity",
        EnergyType::Gas => "gas",
    };

    let reading_type_str = match reading_type {
        ReadingType::Consumption => "consumption",
        ReadingType::Tariff => "tariff",
    };

    let url_base = format!("{BASE_URL}/{energy_type_str}/{reading_type_str}/1");

    let url = Url::parse_with_params(
        &url_base,
        [
            ("start", start.format("%Y%m%d").to_string()),
            ("end", end.format("%Y%m%d").to_string()),
            ("output", "json".to_string()),
        ],
    )?;

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("Authorization", /* get secret here*/)
        .send()
        .await?;

    println!("{:?}", resp);

    let resp_json = resp.json::<Value>().await?;

    if let Some(values) = resp_json.get("values").and_then(|v| v.as_array()) {
        let value_objects: Vec<T> = values
            .iter()
            .filter_map(|x| serde_json::from_value::<T>(x.clone()).ok())
            .collect();

        return Ok(value_objects);
    }

    Err(GetRecordsError::Custom("Could not parse data".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    /*
    #[test]
    fn test_deserialize_electricity_consumption() {
        let json_data = r#"{"timestamp":"2024-08-09 01:00","value":0.038}"#;
        let expected = ElectricityConsumptionDto {
            timestamp: NaiveDateTime::parse_from_str("2024-08-09 01:00", "%Y-%m-%d %H:%M").unwrap(),
            value: 0.038,
        };

        let deserialized: ElectricityConsumptionDto = serde_json::from_str(json_data).unwrap();
        assert_eq!(deserialized, expected);
    }*/
}
