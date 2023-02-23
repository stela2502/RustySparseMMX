use clap::Parser;

use std::{ fs, io, path::PathBuf };
//use std::collections::BTreeMap;

use std::io::BufRead;
use std::path::Path;
use std::io::BufReader;
use std::io::Read;

use flate2::write::GzDecoder;


use crate::sparsedata::SparseData;
use regex::Regex;
use ascii::{AsciiString, FromAsciiError};


pub mod sparsedata;

//use crate::SparseData;

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

    for path in fs::read_dir(root)? {
        let path = path?.path();
        //println!("{path:?}");
        if re.is_match( path.to_str().unwrap() ){
            //println!("pushing");
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
    let mut reader = std::io::BufReader::new(fi);

    let mut data =SparseData::new();
    
    for line in reader.lines() {
        data.add_data( line.unwrap().split( sep ).collect() );
    }
    
    data
}

fn process_file_gz_ascii( file:&PathBuf, sep:char ) -> SparseData {
    let fi = std::fs::File::open( file ).unwrap();
    let gz = GzDecoder::new(fi);
    let mut reader = BufReader::new(gz);

    println!("I am processing the ascii data");

    let mut data =SparseData::new();

    let mut buffer:String = String::new();
    reader.read_to_string( &mut buffer ).unwrap();
    
    for line in buffer.lines() {
        data.add_data( line.split( sep ).collect() );
    }
    data
}


fn process_file_gz( file:&PathBuf, sep:char) -> SparseData {

    let fi = std::fs::File::open( file ).unwrap();
    let gz = GzDecoder::new(fi);
    let reader = std::io::BufReader::new(gz);

    let mut data =SparseData::new();

    panic!("Sorry gz files are not supported here");
    // for line in reader.lines() {
    //     match line {
    //         Ok(line) => {
    //             data.add_data( line.split( sep ).collect() );
    //         },
    //         Err(err) => {
    //             data.add_data( line.as_utf8().split( sep ).collect() );
    //             // if err.kind() == std::io::ErrorKind::InvalidData {
    //             //     // asume ascii here!
    //             //     return process_file_gz_ascii( file, sep );
    //             // }else {
    //             //     panic!("Unexpected error reading the gz file: {err:?}");
    //             // }
    //         }
    //     };  
    // }
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
        
        let content:[usize;3] = data.content(); 

        println!("{} columns {} rows and {} data points read", content[0], content[1], content[2] );

        let ofile = Path::new( opts.ipath.as_str() ).join( &f.file_stem().unwrap() );

        data.write_2_path( ofile, opts.transpose != "false"  ).unwrap();

        println!("finished with {f:?}");
    }

}
