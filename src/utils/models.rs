use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryRecord {
    #[serde(rename = "Country")]
    pub country: Option<String>,

    #[serde(rename = "ISO3")]
    pub iso3: Option<String>,

    #[serde(rename = "Year")]
    pub year: Option<u16>,

    #[serde(rename = "Population")]
    pub population: Option<u64>,

    #[serde(rename = "Population Growth")]
    pub population_growth: Option<i64>,

    #[serde(rename = "Growth Rate (%)")]
    pub growth_rate: Option<f64>,

    #[serde(rename = "Decade")]
    pub decade: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReadQuery {
    #[serde(flatten)]
    pub query: QueryType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QueryType {
    Country {
        #[serde(default)]
        name: Option<String>,

        #[serde(default)]
        iso3: Option<String>,
    },
    TopGrowth {
        year: u16,
        #[serde(default)]
        limit: Option<usize>,
    },
    GlobalTrends {
        #[serde(default)]
        decade: Option<String>,
    },
    CompareCountries {
        countries: Vec<String>,
    },
    PopulationSummary {
        period: TimePeriod,
        metrics: Vec<Metrics>,
    },
    PeakGrowthYear {
        country: String,
    },
    DoublingTimeEstimate {
        country: String,
        #[serde(default)]
        year: Option<u16>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimePeriod {
    Year,
    Decade,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Metrics {
    Mean,
    Median,
    Min,
    Max,
}
