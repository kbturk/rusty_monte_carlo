//https://gitlab.com/derrickturk/pp4e/-/blob/master/src/05/case_study/sim.py
/*

//# of Trials
let TRIALS: i32 = 10000;

//discount rate
let DISC: f64 = 0.10;

enum PRICE_DECKS{
    flat: PiceDeck(
        oil_price=[40.0*36],
        gas_price=[2.50*36]
    )
}
*/
use std::{env, error::Error, fs::File, ffi::OsString, io::BufReader, io::prelude::*};
//use rusty_mc::*;
use serde_json::{Value};
use serde::Deserialize;

fn get_args(n:usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(n){
            Some(file_path) => Ok(file_path),
            None => { 
                let msg = format!("expected csv argument at {}, but got none. 
            Please supply a tree JSON file.", n);
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
    let v:Value = serde_json::from_reader(reader)?;
    println!("{:?}", v);

    Ok(())

}