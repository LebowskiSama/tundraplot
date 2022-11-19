use std::sync::{Arc, Mutex};

use crate::dtypes::dataframes::AccDataFrame;

macro_rules! sanitize_parse {
    ($s: expr) => {
        $s.chars().filter(|c| c.is_digit(10)).collect::<String>().to_owned().parse::<i32>().unwrap()
    };
}

pub fn parse(store: &mut Arc<Mutex<Vec<Vec<String>>>>, df: &mut Arc<Mutex<AccDataFrame>>) {
    //! picks the last sample from a growing vector of data stream
    //! sanitizes and parses sample
    //! appends into dataframe
    //! 
    //! accepts:
    //! -------
    //! store: 2-D vector of strings
    //! df: custom AccDataframe struct

    let store_ = store.lock().unwrap();

    match store_.last() {
        Some(latest) => parse_into_df(&latest, df),
        None => (),
    }
}

fn parse_into_df(latest: &Vec<String>, df: &mut Arc<Mutex<AccDataFrame>>) {
    //! sanitize strings and parse to int
    //! push timestamps, x, y, z into dataframe
    //! 
    //! accepts
    //! -------
    //! latest: 1-D vector of Strings
    //! df: custom AccDataframe struct

    let t = sanitize_parse!(latest[1]);
    let x = sanitize_parse!(latest[2]);
    let y = sanitize_parse!(latest[3]);
    let z = sanitize_parse!(latest[4]);
    
    let mut df = df.lock().unwrap();
    df.push(t, x, y, z)
}