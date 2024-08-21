use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
pub struct GasConsumption {
    pub timestamp: NaiveDateTime,
    pub value: f64,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct ElectricityConsumption {
    pub timestamp: NaiveDateTime,
    pub value: f64,
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use chrono::NaiveDateTime;

    use crate::{
        internal::consumption::{ElectricityConsumptionDto, GasConsumptionDto},
        ElectricityConsumption, GasConsumption,
    };

    #[test]
    fn test_deserialize_electricity_consumption() {
        let json_data = r#"{"timestamp":"2024-08-09 01:00","value":0.038}"#;
        let expected = ElectricityConsumption {
            timestamp: NaiveDateTime::parse_from_str("2024-08-09 01:00", "%Y-%m-%d %H:%M").unwrap(),
            value: 0.038,
        };

        let deserialized: ElectricityConsumption =
            serde_json::from_str::<ElectricityConsumptionDto>(json_data)
                .unwrap()
                .borrow()
                .try_into()
                .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_deserialize_gas_consumption() {
        let json_data = r#"{"timestamp":"2024-08-09 01:00","value":3.5}"#;
        let expected = GasConsumption {
            timestamp: NaiveDateTime::parse_from_str("2024-08-09 01:00", "%Y-%m-%d %H:%M").unwrap(),
            value: 3.5,
        };

        let deserialized: GasConsumption = serde_json::from_str::<GasConsumptionDto>(json_data)
            .unwrap()
            .borrow()
            .try_into()
            .unwrap();
        assert_eq!(deserialized, expected);
    }
}
