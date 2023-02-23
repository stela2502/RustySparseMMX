use std::collections::BTreeMap;

use std::fs::File;
use std::fs;


use std::io::BufWriter;
use flate2::Compression;
use flate2::write::GzEncoder;

use std::io::Write;
use std::path::PathBuf;
use regex::Regex;


struct Data{
	row_id: usize,
	data: BTreeMap<usize, usize>, // cell id => count
}

impl Data{
	pub fn new( row_id:usize ) -> Self{
		let data =  BTreeMap::<usize, usize>::new();
		Self{
			row_id,
			data
		}
	}
	pub fn add( &mut self, cell_id: usize, val: usize ){

		if val > 0 {

			match &self.data.insert( cell_id, val ){
				Some(v) => { 
					panic!("the value is not new: {val} old: {cell_id} -> {v}")
				},
				None => () , // all good
			};
		}
	}
	pub fn to_str( &self , transpose:bool ) -> String {
		let mut ret = "".to_string();
		if transpose{
			for ( col_id, val ) in &self.data{
				ret += &format!( "{} {} {}\n", col_id, self.row_id, val );
			}
		}
		else {
			for ( col_id, val ) in &self.data{
				ret += &format!( "{} {} {}\n", self.row_id, col_id, val );
			}
		}
		ret.pop();
		ret
	}
}

pub struct SparseData{    
    header: BTreeMap<usize, String>, // the col names
    rows:  BTreeMap<usize, String>, // the row names
    data: BTreeMap<usize, Data>, // the Data
    row_id: usize, // the local gene id
    counts: usize, // how many values were stored?
    counts_r: usize, // count for the rows
    counts_c: usize, // cout for the columns
    first:bool, // is this the first value added?
}

impl SparseData{

	pub fn new() ->Self {
		let header = BTreeMap::<usize, String>::new();
		let rows = BTreeMap::<usize, String>::new();
		let data = BTreeMap::<usize, Data>::new();
		let row_id = 1;
		let counts = 0;
		let counts_r = 0;
		let counts_c= 0;
		let first = true;
		Self{
			header,
			rows,
			data,
			row_id,
			counts,
			counts_r,
			counts_c,
			first
		}
	}

	pub fn add_header(&mut self,  names:Vec<&str> ) {
		let mut id = 1;
		// first is a blank
		let mut first = true;
		for cell_name in names {
			if first{
				first = false;
				continue;
			}
			//println!("I insert col '{cell_name}'" );
			self.header.insert( id, cell_name.to_string() );
			id += 1;
		}
		//println!("I have detected {} columns", self.header.len() );
	}

	pub fn add_row( &mut self, val:String ){
		self.counts_r += 1;
		self.rows.insert( self.counts_r, val );
	}

	pub fn add_col( &mut self, val:String ){
		self.counts_c += 1;
		self.header.insert( self.counts_c, val );
	}

	pub fn add_data (&mut self,  dat:Vec<&str> ){

		if self.first {
			self.first = false;
			return self.add_header( dat );
		}
		let mut col_id = 0;

		let mut row_good = true;


		for x in dat.iter() {
			if row_good{
				self.rows.insert(self.row_id, x.to_string());
				self.row_id += 1;
				row_good = false;
			}
			else {
				col_id +=1;
				let val = match x.parse::<usize>() {
					Ok( v ) => v,
					Err(_err) => {
						match x.parse::<f32>(){
							Ok(v) =>  { 
								let r= v.round() ;
								let ret = r as usize;
								ret
							},
							Err(_err) => {
								//eprintln!("I could not parse '{x}' to usize or f32 {err:?}");
								0
							},
						}
					},
				};
				if val > 0 {
					match self.data.get_mut( &(&self.row_id-1)  ) {
						Some( row ) => {
							row.add( col_id, val );
						}
						None => {
							let mut row = Data::new( &self.row_id-1 );
							row.add( col_id, val );
							self.data.insert(self.row_id-1 , row );
						}
					};
					self.counts += 1;
				}
				
			}
		}
		//println!("I read {} rows {} columns and {} entries != 0", self.rows.len(), self.header.len(), self.counts);
	}

	pub fn add_alevin_sparse( &mut self, dat:Vec<&str> ){

		//panic!("add_alevin_sparse got this: {dat:?}");

		let col_id = match dat[0].parse::<usize>() {
			Ok( v ) => v,
			Err(_err) => {
				match dat[0].parse::<f32>(){
					Ok(v) =>  { 
						let r= v.round() ;
						let ret = r as usize;
						ret
					},
					Err(_err) => {
						//eprintln!("I could not parse '{x}' to usize or f32 {err:?}");
						0
					},
				}
			},
		};

		if col_id != 0 && self.first { // this is the header line
			self.first = false;
			return ();
		}
		if col_id == 0 {
			return ();
		}
		let row_id = match dat[1].parse::<usize>() {
			Ok( v ) => v,
			Err(_err) => {
				match dat[1].parse::<f32>(){
					Ok(v) =>  { 
						let r= v.round() ;
						let ret = r as usize;
						ret
					},
					Err(_err) => {
						//eprintln!("I could not parse '{x}' to usize or f32 {err:?}");
						0
					},
				}
			},
		};

		let val = match dat[2].parse::<usize>() {
			Ok( v ) => v,
			Err(_err) => {
				match dat[2].parse::<f32>(){
					Ok(v) =>  { 
						let r= v.round() ;
						let ret = r as usize;
						ret
					},
					Err(_err) => {
						//eprintln!("I could not parse '{x}' to usize or f32 {err:?}");
						0
					},
				}
			},
		};

		match self.data.get_mut( &row_id  ) {
			Some( row ) => {
				row.add( col_id, val );
			}
			None => {
				let mut row = Data::new( row_id );
				row.add( col_id, val );
				self.data.insert(row_id , row );
			}
		};
		self.counts += 1;
	}

	pub fn write_2_path( &self, main_path:PathBuf, transpose:bool ) -> Result< (), &str> {


		let re = Regex::new(r"\s+").unwrap();

        if ! &main_path.exists() {
            match fs::create_dir ( main_path.clone() ){
                Ok(_file) => (),
                Err(err) => {
                     eprintln!("Error?: {err:#?}");
                 }
            };
        }

        let file_path = main_path.join( "filtered_feature_bc_matrix" );

        if ! &file_path.exists() {
            match fs::create_dir ( file_path.clone() ){
                Ok(_file) => (),
                Err(err) => {
                     eprintln!("Error?: {err:#?}");
                 }
            };
        }

        ////////////////////////////////////////////////////////////////////
        //  barcodes  //
        ////////////////////////////////////////////////////////////////////

        if  fs::remove_file(file_path.join("barcodes.tsv.gz") ).is_ok(){};

        let file_b = match File::create( file_path.join("barcodes.tsv.gz") ){
            Ok(file) => file,
            Err(err) => {
                panic!("Error creating the path?: {err:#?}");
            }
        };
        let file2 = GzEncoder::new(file_b, Compression::default());
        let mut writer_b = BufWriter::new(file2);


        if transpose {
        	for name in self.rows.values() {
	        	let na = &re.replace_all( name, "_");
	            match writeln!( writer_b, "{na}"){
	                Ok(_) => (),
	                Err(err) => {
	                    eprintln!("write error: {err}" );
	                    return Err::<(), &str>("feature could not be written")   
	                }
	            }
	        }
        }else{
	        for name in self.header.values() {
	        	let na = &re.replace_all( name, "_");
	            match writeln!( writer_b, "{na}"){
	                Ok(_) => (),
	                Err(err) => {
	                    eprintln!("write error: {err}" );
	                    return Err::<(), &str>("feature could not be written")   
	                }
	            }
	        }
	    }

        ////////////////////////////////////////////////////////////////////
        //  features  //
        /////////////////////////////////////////////////////////////////////

        if fs::remove_file(file_path.join("features.tsv.gz") ).is_ok(){};
 
        let file_f = match File::create( file_path.join("features.tsv.gz") ){
            Ok(file) => file,
            Err(err) => {
                panic!("Error creating the path?: {err:#?}");
            }
        };
        let file3 = GzEncoder::new(file_f, Compression::default());
        let mut writer_f = BufWriter::new(file3);

        if transpose {
        	for  name in self.header.values() {
	        	//let &mut na = name.to_string();
	        	let na = &re.replace_all( name, "_");
	            match writeln!( writer_f, "{na}\t{na}\tGene Expression"  ){
	                Ok(_) => (),
	                Err(err) => {
	                    eprintln!("write error: {err}" );
	                    return Err::<(), &str>("feature could not be written")   
	                }
	            }
	        }
        }else {
        	for  name in self.rows.values() {
	        	//let &mut na = name.to_string();
	        	let na = &re.replace_all( name, "_");
	            match writeln!( writer_f, "{na}\t{na}\tGene Expression"  ){
	                Ok(_) => (),
	                Err(err) => {
	                    eprintln!("write error: {err}" );
	                    return Err::<(), &str>("feature could not be written")   
	                }
	            }
	        }
        }
        


        ////////////////////////////////////////////////////////////////////
        //  matrix  //
        /////////////////////////////////////////////////////////////////////


        if fs::remove_file(file_path.join("matrix.mtx.gz") ).is_ok(){};

        let file = match File::create( file_path.join("matrix.mtx.gz") ){
            Ok(file) => file,
            Err(err) => {
                panic!("Error creating the path?: {err:#?}");
            }
        };
        let file1 = GzEncoder::new(file, Compression::default());
        let mut writer = BufWriter::new(file1);

        match writeln!( writer, "%%MatrixMarket matrix coordinate integer general\n{}", 
             self.mtx_counts( transpose ) ){
            Ok(_) => (),
            Err(err) => {
                eprintln!("write error: {err}");
                return Err::<(), &str>("Header could not be written")
            }
        };

        ////////////////////////////////

        let mut entries = 0;
        for row in self.data.values() {
            match writeln!( writer, "{}", row.to_str( transpose) ){
                Ok(_) => {entries += 1;},
                Err(err) => {
                    eprintln!("write error: {err}");
                    return Err::<(), &str>("cell data could not be written")   
                }   
            };

        }
        println!( "sparse Matrix: {} cell(s), {} gene(s) and {} entries written to path {:?}; ", self.header.len(), self.rows.len(), entries, file_path.into_os_string().into_string());
        Ok(())
	}

	pub fn content(&self)  -> [usize;3] {
		return [ self.rows.len(), self.header.len(), self.counts ];
	}

	pub fn mtx_counts(&self, transpose:bool) -> String{
		let content:[usize;3] = self.content(); 
		if transpose{
			return format!("{} {} {}",  content[1], content[0], content[2] )
		} 
		format!("{} {} {}",  content[0], content[1], content[2] )
	}
	
}
