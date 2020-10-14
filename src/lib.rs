//https://gitlab.com/derrickturk/pp4e/-/blob/master/src/05/case_study/datatypes.py
//! 'rusty-mc: lib.rs' is a library for calculating a Monte Carlo forecasting tool to estimate oil well production.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter };
use rand::{distributions::WeightedIndex, prelude::*};

///Maps the project in JSON file in main to a project. The default projects are: Bat country, Needs more Bats, Bat Classic, Beyond the Pale, & High Water Mark.
///Bonus points if you know what movie these quotes are from.
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    #[serde(rename = "oil_shrink")]
    pub oil_shrink_factor: f64,
    #[serde(rename = "gas_shrink")]
    pub gas_shrink_factor: f64,
    #[serde(rename = "wi")]
    pub working_interest: f64,
    #[serde(rename = "nri")]
    pub net_revenue_interest: f64,
    pub tax_rate: f64, //federal+severance+ad valorem tax rate
    pub is_root: Option <bool>,
    pub outcomes: Vec <Casedata>,

}

///Monthly case data for cases. Low, Most Likely, High, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Casedata {
    pub probability: f64,
    #[serde(rename = "leads_to")]
    pub outcome_projects: Vec <String>,
    #[serde(rename = "oil")]
    pub gross_oil: Vec <f64>,
    #[serde(rename = "gas")]
    pub gross_gas: Vec <f64>,
    #[serde(rename = "capex")]
    pub monthly_capital: Vec <f64>,
    #[serde(rename = "opex")]
    pub monthly_operating_expenses: Vec <f64>,

}

#[derive(Debug)]
pub struct Stats {
    mean: f64,
    p90: f64,
    p50: f64,
    p10: f64,
}

#[derive(Debug)]
pub struct PriceDeck {
    oil_price: Vec <f64>,
    gas_price: Vec <f64>,
}

pub fn price_deck() -> HashMap<String, PriceDeck> {
    let mut pd = HashMap::new();
    pd.insert(String::from("flat"), PriceDeck{
        oil_price: iter::repeat(40.0).take(36).collect(), 
        gas_price: iter::repeat(2.5).take(36).collect(),
        });

    pd.insert(String::from("high"), PriceDeck{
        oil_price: iter::repeat(40.0).take(12).chain(iter::repeat(60.0).take(12)).chain(iter::repeat(90.0).take(12)).collect(),
        gas_price: iter::repeat(2.5).take(12).chain(iter::repeat(3.5).take(12)).chain(iter::repeat(5.0).take(12)).collect(),
        });

    pd.insert(String::from("low"), PriceDeck{
        oil_price: iter::repeat(40.0).take(12).chain(iter::repeat(20.0).take(24)).collect(), 
        gas_price: iter::repeat(2.50).take(12).chain(iter::repeat(2.00).take(24)).collect(),
        });

    return pd
}

impl Project {

    ///returns gross_revenue, which is a vector from [0..=35] months
    ///Inputs: a pointer to a project, j, the chosen outcome to analyze, and a pointer to a price deck.
    pub fn gross_revenue(&self, j:usize, price_deck: &PriceDeck) -> Vec <f64> {

        (0..=35).map(|i| self.net_revenue_interest * ((1.0 - self.oil_shrink_factor) * 
        self.outcomes[j].gross_oil[i] * price_deck.oil_price[i] + (1.0 - self.gas_shrink_factor) * 
        self.outcomes[j].gross_gas[i] * price_deck.gas_price[i])).collect()

    }

    ///returns net_capital_expense, a vector from [0..=35] months
    ///Inputs: a pointer to a project and j, the chosen outcome to analyze.
    pub fn net_capital_expense (&self, j:usize) -> Vec<f64> {
        self.outcomes[j].monthly_capital.iter().map(|x| x* self.working_interest * 1000.0).collect()
    }

    ///returns monthly_operating_expenses, a vector from [0..=35] months
    ///Inputs: a pointer to a project and j, the chosen outcome to analyze.
    pub fn net_operating_expense (&self, j:usize) -> Vec<f64> {
        self.outcomes[j].monthly_operating_expenses.iter().map(|x| x * self.working_interest * 1000.0).collect()
    }

    ///returns monthly tax, a vector from [0..=35] months
    ///Inputs: monthly gross_revenue(Project, j, price_deck) * tax_rate
    pub fn tax (&self, j:usize, price_deck:&PriceDeck) -> Vec<f64> {
        self.gross_revenue(j, price_deck).iter().map(|x| x * self.tax_rate).collect()
    }

    ///Returns net_cf, a vector from [0..=35] months of cashflow.
    pub fn net_cash_flow(&self, j:usize, price_deck: &PriceDeck) -> Vec<f64> {
        (0..=35).map(|i| self.gross_revenue(j, price_deck)[i] - 
        self.net_capital_expense(j)[i] - self.net_operating_expense(j)[i] - self.tax(j,price_deck)[i]).collect()
    }

    ///Returns the net present value of a series of cashflows at a given discount rate.
    pub fn npv(&self, j:usize, price_deck: &PriceDeck, discount: f64) -> f64{
        let mut i:f64 = 0.0;
        self.net_cash_flow(j, price_deck).iter().map(|x| {i += 1.0; x / (1.0 + discount).powf(i / 12.0) }).sum()
    }

    ///This will select a random usize number, j, for the monte-carlo simulator.
    pub fn outcome_selector(&self) -> usize {
        let mut rng = thread_rng();
        let choice = WeightedIndex::new(self.outcomes.iter().map(|o| o.probability)).unwrap();
        choice.sample(&mut rng)
    }

}

///Create a HashMap of project names, projects. This will allow easy indexing into projects.
///Must be public so it can be called from main.rs.
pub fn project_list(v: &Vec<Project>) -> HashMap<&String, &Project>{
    let mut pl = HashMap::new();
    for project in v{
        pl.insert(&project.name, project);
    }
    pl
}

fn convert_vec_string_to_projects<'a>(string_project_vec: &Vec<String>, 
pl: &HashMap<&String, &'a Project>) -> Vec<&'a Project>{
    let mut rp = Vec::new();
    for item in string_project_vec {
        rp.push(*pl.get(&item).unwrap());
    }
    rp
}

///Create a vector of selected tuples of (project, j) for monte-carlo calculations.
///j is a usize used to index a selected Casedata in a Project.
pub fn selected_outcome_group<'a>(root_projects: &Vec<String>,
    pl: &HashMap<&String, &'a Project>) -> Vec<(&'a Project, usize) > {
    
    let mut r = Vec::new();
    let rp = convert_vec_string_to_projects(root_projects, pl);

    for project in rp{
        let mut j:usize = project.outcome_selector();
        r.push((project, j));

        //TODO: Add some form of recursion instead of rewriting the selected_outcome_group.
       let sp = convert_vec_string_to_projects(&project.outcomes[j].outcome_projects, pl);

       for subsequent_project in sp {
            j = subsequent_project.outcome_selector();
            r.push((subsequent_project,j)); 
        }
    }
    return r
}

///Run monte-carlo trials & return Vec<T>.
pub fn monte_carlo_trials<T, F: Fn(Vec<(&Project, usize)>) -> T>
(n:i64, root_projects: &Vec<String>, pl: &HashMap<&String, &Project>, 
stat_fn: F ) -> Vec<T> {

    let mut r = Vec::new();
    for _ in 0..n {
        r.push(stat_fn(selected_outcome_group(root_projects, pl)));
        
    }
    r
}

///Stat calculations. This returns a mean, p90, p50, and p10 of value f64.
pub fn extract_stats(xs: &mut Vec<f64>) -> Stats {
    let n = xs.len() as f64;
    xs.sort_by(|a,b| a.partial_cmp(b).unwrap());
    
    return Stats{
        mean: xs.iter().map( |x| *x / n ).sum(),
        p90: xs[((n-1.0)*0.1).round() as usize],
        p50: xs[((n-1.0)*0.5).round() as usize],
        p10: xs[((n-1.0)*0.9).round() as usize],
    }
}