use csv::{Error, Reader};
use std::{fmt, fs::File, io, result};
use ndarray::{ Array, Array1, Array2 };
use linfa::Dataset;


struct SliceDisplay<'a, T: 'a>(&'a [T]);

impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}


fn get_headers(reader: &mut Reader<File>) -> Vec<String> {
return reader
    .headers().unwrap().iter()
    .map(|r| r.to_owned())
    .collect();
}

fn get_records(data: &Vec<Vec<usize>>, target_index: usize) -> Array2<usize> {
let mut records: Vec<usize> = vec![];
for record in data.iter() {
    records.extend_from_slice( &record[0..target_index] );
}
return Array::from( records ).into_shape((303, 13)).unwrap();
}

fn get_targets(data: &Vec<Vec<usize>>, target_index: usize) -> Array1<usize> {
let targets = data
    .iter()
    .map(|record| record[target_index] as usize)
    .collect::<Vec<usize>>();
    return Array::from( targets );
}

fn get_data(reader: &mut Reader<File>) -> Vec<Vec<usize>> {
return reader
    .records()
    .map(|r|
    r
        .unwrap().iter()
        .map(|field| field.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
    )
    .collect::<Vec<Vec<usize>>>();
}

fn get_dataset() -> Dataset<usize, usize, ndarray::Dim<[usize; 1]>> {
    let mut reader = Reader::from_path("./src/heart_data1.csv").unwrap();

    let headers = get_headers(&mut reader);
    println!("{}", SliceDisplay(&headers));

    let data = get_data(&mut reader);
    let target_index = headers.len() - 1;

    let features = headers[0..target_index].to_vec();
    let records = get_records(&data, target_index);
    let targets = get_targets(&data, target_index);

    for data in headers{
        print!("{}",data);
    }
    return Dataset::new(records, targets)
        .with_feature_names(features);
}

fn main() {
    get_dataset();
}
