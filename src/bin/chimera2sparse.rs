use clap::Parser;

use std::{ fs, io, path::PathBuf };
//use std::collections::BTreeMap;

use std::io::BufRead;
use std::path::Path;

use flate2::read::GzDecoder;


use regex::Regex;
//use ascii::{AsciiString, FromAsciiError};
 use sparsedata::sparsedata::SparseData;

//use regex::Regex;
//use ascii::{AsciiString, FromAsciiError};



/// Convert chimeric mess of a sparse table to CellRangers MatrixMarket format.
/// The in data is a merge of barcodes and features info followed by the expression value for the specific combination.
/// In other words a total waste of storage space.
/// Meaning the outfiles are matrix.mtx.gz, features.tsv.gz and barcodes.tsv.gz.
/// To circumvent problems while importing into Scanpy the files are created in a folder named 'filtered_feature_bc_matrix'.
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the input input path
    #[clap(short, long)]
    ipath: String,
    /// the column separator str
    #[clap(default_value=",", short, long)]
    sep: String,
    /// transpose the data
    #[clap(default_value="false", short, long)]
    transpose:String,
}



/// from https://stackoverflow.com/questions/51418859/how-do-i-list-a-folder-and-return-all-the-file-names-of-a-specific-file-type
fn list_of_csv_paths(root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];
    let re = Regex::new("csv.?g?z?$").unwrap();
    //let re = Regex::new("csv$").unwrap();

    for path in fs::read_dir(root)? {
        let path = path?.path();
        //println!("{path:?}");
        if re.is_match( path.to_str().unwrap() ){
            //println!("pushing {path:?}");
            result.push(path.to_owned());
        }
        //if let Some("csv") = path.extension().and_then(OsStr::to_str) {
        //    result.push(path.to_owned());
        //}
    }
    Ok(result)
}

fn process_file( file:&PathBuf, sep:char ) -> SparseData {


    let fi = std::fs::File::open( file ).unwrap();
    let reader = std::io::BufReader::new(fi);

    let mut data =SparseData::new();
    
    for line in reader.lines() {
        match line {
            Ok(line) => {
                data.add_chimera( line.split( sep ).collect() );
            },
            Err(err) => {
                panic!("Unexpected error reading the csv file: {err:?}");
            }
        }
    }
    let con = data.content();
    println!("I have read {} rows and {} cols and {} values", con[1], con[0], con[2] );
    data
}


fn process_file_gz( file:&PathBuf, sep:char) -> SparseData {

    let fi = std::fs::File::open( file ).unwrap();
    let gz = GzDecoder::new(fi);
    let reader = std::io::BufReader::new(gz);

    let mut data =SparseData::new();

    //eprintln!("Sorry gz files are not supported here {file:?} {sep:?}");
    for line in reader.lines() {
        match line {
            Ok(line) => {
                data.add_chimera( line.split( sep ).collect() );
            },
            Err(err) => {
                panic!("Unexpected error reading the gz file: {err:?}");
                //data.add_data( line.as_utf8().split( sep ).collect() );
                // if err.kind() == std::io::ErrorKind::InvalidData {
                //     // asume ascii here!
                //     return process_file_gz_ascii( file, sep );
                // }else {
                //     panic!("Unexpected error reading the gz file: {err:?}");
                // }
            }
        };  
    }
    data
}

fn main() {

    let opts: Opts = Opts::parse();
    let re2 = Regex::new("gz$").unwrap();
    let mut sep = '\t';
    if &opts.sep != "\\t"{
        sep = opts.sep.chars().next().unwrap(); 
    }

    for f in list_of_csv_paths( &opts.ipath ).unwrap(){

        println!("Processing file {:?}", f);

        let data = match re2.is_match( f.to_str().unwrap()  ){
            true  => {
                process_file_gz( &f , sep )
            }
            false => {
                process_file( &f , sep )
            }
        };
        
        //let content:[usize;3] = data.content(); 
        //println!("{} columns {} rows and {} data points read", content[0], content[1], content[2] );
        
        let path_str = &f.file_stem().unwrap().to_str().unwrap();
        //println!("{path_str}");

        let ofile = match  path_str.strip_suffix(".csv"){
            Some(n) => {
                Path::new( opts.ipath.as_str() ).join( n )
            },
            None => Path::new( opts.ipath.as_str() ).join( path_str),
        };

        data.write_2_path( (ofile).to_path_buf(), opts.transpose != "false"  ).unwrap();

        println!("finished with {f:?}");
    }

}
