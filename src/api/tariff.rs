use chrono::{NaiveDate, NaiveDateTime};

pub struct StandingCharge {
    pub start_date: NaiveDate,
    pub value: f64,
}

pub struct ElectricityTariff {
    pub standing_charges: Vec<StandingCharge>,
    pub prices: Vec<TimestampedValue>,
}

pub struct GasTariff {
    pub standing_charges: Vec<StandingCharge>,
    pub prices: Vec<TimestampedValue>,
}

pub struct TimestampedValue {
    pub timestamp: NaiveDateTime,
    pub value: f64,
}
