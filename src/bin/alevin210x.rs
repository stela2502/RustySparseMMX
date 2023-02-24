use clap::Parser;


use std::io::BufRead;
use std::path::Path;

use sparsedata::sparsedata::SparseData;


/// alevin-fry (a single cell quantification tool written in Rust) creates MatrixMarket
/// outfiles that are not conform with the 10x Cellranger standard. This format is not supported
/// by the main analyis packages and therfore this tool converts alevin-fry style matrices to
/// CellRanger style MatrixMarket format.

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
    	let reader_c = std::io::BufReader::new(fi);
    	for line in reader_c.lines() {
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
    	let reader_r = std::io::BufReader::new(fi);
    	for line in reader_r.lines() {
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
    	let reader_m = std::io::BufReader::new(fi);
    	for line in reader_m.lines() {
    		data.add_alevin_sparse( line.unwrap().split( ' ' ).collect()  )
    	}
    }
   	else{
   		panic!("missing file quants_mat.mtx");
   	}

    data.write_2_path( (*Path::new(&opts.ipath)).to_path_buf() , false ).unwrap();

}
