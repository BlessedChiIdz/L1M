use csv::{Error, Reader};
use std::{fs::File, io, result};
use ndarray::{ Array, Array1, Array2 };
use linfa::Dataset;


fn get_dataset() {
    let mut rdr = Reader::from_path("./src/heart_data.csv").unwrap();
   
    for result in rdr.records() {
        let record = result;
        println!("{:?}", record);
    }

    let headers = rdr.headers();
    println!("{:?}", headers);

   }

fn main() {
    get_dataset();
}
