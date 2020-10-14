//https://gitlab.com/derrickturk/pp4e/-/blob/master/src/05/case_study/sim.py

use std::{env, error::Error, fs::File, ffi::OsString, io::BufReader };
use rusty_mc::*;

const TRIALS: i64 = 1000;
const DISCOUNT: f64 = 0.10;

fn get_args(n:usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(n){
            Some(file_path) => Ok(file_path),
            None => { 
                let msg = format!("expected csv argument at {}, but got none. Please supply a tree JSON file.", n);
            Err(msg)?
        },
    }
} 

fn main() -> Result<(), Box<dyn Error>>{

    //Find the JSON file.
    let file_path = get_args(1)?;
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    
    //Read the JSON file.
    let v: Vec<Project> = serde_json::from_reader(reader)?;
    
    //Make the Hashmap of all names, projects.
    let pl = project_list(&v);
    
    //Choose the root projects.
    let root_projects=vec![String::from("BAT COUNTRY"), String::from("BEYOND THE PALE")];

    //TEMP: Generate the "roll of the die."
    //selected_outcome_group(&root_projects, &pl);

    //Run the Monte-Carlo simulation.
    let price_deck = price_deck();
    
    for (name, deck) in price_deck{
        let mut npvs = 
        monte_carlo_trials( TRIALS, &root_projects, &pl, 
        |x| x.iter().map(|(project, selected_outcome)| {
            project.npv(*selected_outcome, &deck, DISCOUNT )
        }).sum::<f64>());
        
        println!("{:?}, {:?}",name, npvs);
        println!("{:#?}", extract_stats(&mut npvs));
    }
    
    Ok(())

}