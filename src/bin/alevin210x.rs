use clap::Parser;

use std::{ fs, io, path::PathBuf };
//use std::collections::BTreeMap;

use std::io::BufRead;
use std::path::Path;


//use std::io::BufReader;
//use std::io::Read;
//use flate2::write::GzDecoder;


//use dense2sparse::sparsedata::SparseData;

use regex::Regex;
//use ascii::{AsciiString, FromAsciiError};
use sparsedata::sparsedata::SparseData;


#[derive(Parser)]
#[clap(version = "0.1.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the input input path
    #[clap(short, long)]
    ipath: String,
}


fn main() {

    let opts: Opts = Opts::parse();

    let mut data =SparseData::new();

    //quants_mat_cols.txt  

    let cols = Path::new( &opts.ipath).join("quants_mat_cols.txt");
    if cols.exists(){
    	let fi = std::fs::File::open( cols ).unwrap();
    	let readerC = std::io::BufReader::new(fi);
    	for line in readerC.lines() {
    		data.add_row(  line.unwrap());
    	}
    }
   	else{
   		panic!("missing file quants_mat_cols.txt");
   	}

   	//quants_mat_rows.txt

   	let rows = Path::new( &opts.ipath).join("quants_mat_rows.txt");
    if rows.exists(){
    	let fi = std::fs::File::open( rows ).unwrap();
    	let readerR = std::io::BufReader::new(fi);
    	for line in readerR.lines() {
    		data.add_col( line.unwrap());
    	}
    }
   	else{
   		panic!("missing file quants_mat_rows.txt");
   	}


    //quants_mat.mtx

    let mtx = Path::new( &opts.ipath).join("quants_mat.mtx");
    if mtx.exists(){
    	let fi = std::fs::File::open( mtx ).unwrap();
    	let readerM = std::io::BufReader::new(fi);
    	for line in readerM.lines() {
    		data.add_alevin_sparse( line.unwrap().split( ' ' ).collect()  )
    	}
    }
   	else{
   		panic!("missing file quants_mat.mtx");
   	}

    data.write_2_path( (*Path::new(&opts.ipath)).to_path_buf() , false );

}
