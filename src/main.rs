use clap::Parser;

use std::{ fs, io, path::PathBuf, ffi::OsStr };
//use std::collections::BTreeMap;

use std::io::BufRead;
use std::path::Path;

use crate::sparsedata::SparseData;

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
}



/// from https://stackoverflow.com/questions/51418859/how-do-i-list-a-folder-and-return-all-the-file-names-of-a-specific-file-type
fn list_of_csv_paths(root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("csv") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_owned());
        }
    }
    Ok(result)
}

fn process_file( file:&PathBuf, sep:&str) -> SparseData {

    let fi = std::fs::File::open( file ).unwrap();
    let mut reader = std::io::BufReader::new(fi);

    let mut data =SparseData::new();

    let mut line = "".to_string();
    reader.read_line( &mut line).unwrap();
    let vec:Vec<&str> ; //= line.split( '\t' ).collect();

    if sep == "\\t" {
        vec = line.split( '\t' ).collect();
        data.add_header( vec );
        for line in reader.lines() {
            data.add_data( line.unwrap().split('\t').collect() );
        }
    }
    else {
        vec = line.split( sep ).collect();
        data.add_header( vec );
        for line in reader.lines() {
            //let mut line = line.unwrap();
            data.add_data( line.unwrap().split( sep ).collect() );
        }
    }
    
    data
}

fn main() {

    let opts: Opts = Opts::parse();

    for f in list_of_csv_paths( &opts.ipath ).unwrap(){
        println!("Processing file {:?}", f);
        let data = process_file( &f , &opts.sep );
        let content:[usize;3] = data.content(); 

        println!("{} columns {} rows and {} data points read", content[0], content[1], content[2] );

        data.write_2_path( Path::new( opts.ipath.as_str() ).join( &f.file_stem().unwrap() ) ).unwrap();

    }

}
