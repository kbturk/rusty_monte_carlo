//https://gitlab.com/derrickturk/pp4e/-/blob/master/src/05/case_study/datatypes.py
//! #rusty-mc: lib.rs
//!
//! 'rusty-mc: lib.rs' is a library for calculating a Monte Carlo forecasting tool to estimate oil well production.

use serde::{Deserialize, Serialize};

///This is from the area CSVs: Bat country, Needs more Bats, Bat Classic, Beyond the Pale, & High Water Mark.
///Bonus points if you know what movie these quotes are from.
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    #[serde(rename = "Project Name")]
    name: String,
    #[serde(rename = "Oil shrink")]
    oil_shrink_factor: f64,
    #[serde(rename = "Gas shrink")]
    gas_shrink_factor: f64,
    #[serde(rename = "WI")]
    working_interest: f64,
    #[serde(rename = "NRI")]
    net_revenue_interest: f64,
    #[serde(rename = "Tax rate")]
    tax_rate: f64, //federal+severance+ad valorem tax rate
    #[serde(rename = "Depends on")]
    depends_on: Option<String>, //Sometimes None.
    #[serde(rename = "Outcomes")]
    outcome_projects: Option<Vec<String>>, //Sometimes None, sometimes a list.

}

///Monthly case data from CSV files: Low, Most Likely, High, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Casedata {
//    #[serde(rename = "Project Name")]
//    probability: f64, //given in the case study and main CSVs for the different cases.
    #[serde(rename = "Month (from time 0)")]
    month: Vec<f64>, //might change to an integer
    #[serde(rename = "Gross Oil (bbl)")]
    gross_oil: Vec<f64>,
    #[serde(rename = "Gross Gas (mcf)")]
    gross_gas: Vec<f64>,
    #[serde(rename = "Total Capital (M$)")]
    monthly_capital: Vec<f64>,
    #[serde(rename = "Total Operating Expense (M$)")]
    monthly_operating_expenses: Vec<f64>,
}

pub struct PriceDeck {
    oil_price: Vec<f64>,
    gas_price: Vec<f64>,
}

//consider wrapping this in an impl
pub fn gross_revenue(net_revenue_int: f64, oil_shrink_fact: f64, go: f64, oil_price: f64, gas_shrink: f64, gg: f64, gas_price: f64) -> f64 {
    net_revenue_int * ((1.0 - oil_shrink_fact) * go * oil_price + (1.0 - gas_shrink) * gg * gas_price)
}

//consider wrapping this in an impl
pub fn net_capital_expense (capital_expense: f64, working_interest: f64) -> f64 {
    capital_expense * working_interest
}

//consider wrapping this in an impl
pub fn net_operating_expense (operating_expense: f64, working_interest: f64) -> f64 {
    operating_expense * working_interest
}

//consider wrapping this in an impl
pub fn tax (tax_rate:f64, gross_rev:f64) -> f64 {
    tax_rate * gross_rev
}

//consider wrapping this in an impl
pub fn net_cash_flow(gross_rev: f64, net_capital_exp: f64, net_operating_exp: f64, tax_: f64) -> f64 {
    gross_rev - net_capital_exp - net_operating_exp - tax_
}