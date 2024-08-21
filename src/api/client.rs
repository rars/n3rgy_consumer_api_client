use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use serde_json::Value;
use url::Url;

use crate::{
    internal::{
        consumption::{ElectricityConsumptionDto, GasConsumptionDto},
        tariff::{ElectricityTariffDto, GasTariffDto},
    },
    AuthorizationProvider,
};

use super::{
    consumption::ElectricityConsumption, ElectricityTariff, GasConsumption, GasTariff,
    GetRecordsError,
};

const DEFAULT_BASE_URL: &str = "https://consumer-api.data.n3rgy.com";

enum EnergyType {
    Electricity,
    Gas,
}

enum ReadingType {
    Consumption,
    Tariff,
}

pub struct Client<T>
where
    T: AuthorizationProvider,
{
    authorization_provider: T,
    base_url: String,
}

impl<T: AuthorizationProvider> Client<T> {
    pub fn new(authorization_provider: T, base_url: Option<String>) -> Self {
        Self {
            authorization_provider,
            base_url: base_url.unwrap_or(DEFAULT_BASE_URL.into()),
        }
    }

    pub async fn get_electricity_consumption(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<ElectricityConsumption>, GetRecordsError> {
        let dto_records = self
            .get_records::<ElectricityConsumptionDto>(
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

    pub async fn get_electricity_tariff(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<ElectricityTariff>, GetRecordsError> {
        let dto_records = self
            .get_records::<ElectricityTariffDto>(
                EnergyType::Electricity,
                ReadingType::Tariff,
                start,
                end,
            )
            .await?;

        let records = dto_records
            .iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, _>>();

        Ok(records?)
    }

    pub async fn get_gas_consumption(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<GasConsumption>, GetRecordsError> {
        let dto_records = self
            .get_records::<GasConsumptionDto>(EnergyType::Gas, ReadingType::Consumption, start, end)
            .await?;

        let records: Result<Vec<GasConsumption>, chrono::ParseError> =
            dto_records.iter().map(|x| x.try_into()).collect();

        Ok(records?)
    }

    pub async fn get_gas_tariff(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<GasTariff>, GetRecordsError> {
        let dto_records = self
            .get_records::<GasTariffDto>(EnergyType::Electricity, ReadingType::Tariff, start, end)
            .await?;

        let records: Result<Vec<GasTariff>, chrono::ParseError> =
            dto_records.iter().map(|x| x.try_into()).collect();

        Ok(records?)
    }

    async fn get_records<U>(
        &self,
        energy_type: EnergyType,
        reading_type: ReadingType,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<U>, GetRecordsError>
    where
        U: DeserializeOwned,
    {
        let energy_type_str = match energy_type {
            EnergyType::Electricity => "electricity",
            EnergyType::Gas => "gas",
        };

        let reading_type_str = match reading_type {
            ReadingType::Consumption => "consumption",
            ReadingType::Tariff => "tariff",
        };

        let base_url = &self.base_url;

        let url_base = format!("{base_url}/{energy_type_str}/{reading_type_str}/1");

        let url = Url::parse_with_params(
            &url_base,
            [
                ("start", start.format("%Y%m%d").to_string()),
                ("end", end.format("%Y%m%d").to_string()),
                ("output", "json".to_string()),
            ],
        )?;

        let client = reqwest::Client::new();

        let authorization = self.authorization_provider.get_authorization();

        let resp = client
            .get(url)
            .header("Authorization", authorization)
            .send()
            .await?;

        let resp_json = resp.json::<Value>().await?;

        if let Some(values) = resp_json.get("values").and_then(|v| v.as_array()) {
            let value_objects: Vec<U> = values
                .iter()
                .filter_map(|x| serde_json::from_value::<U>(x.clone()).ok())
                .collect();

            return Ok(value_objects);
        }

        Err(GetRecordsError::Custom(
            "Could not retrieve 'values' property from response".into(),
        ))
    }
}
