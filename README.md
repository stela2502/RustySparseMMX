# dense_2_sparse

A rust program to convert a dense ',' or '\t' separated table to the Matrix Marked spasre matrix format.
Only integer values are supported. Float values as provided by some databases are converted to ints.

# Usage

```
dense_2_sparse -h
dense_2_sparse 0.1.0
Stefan L. <stefan.lang@med.lu.se>

USAGE:
    dense_2_sparse [OPTIONS] --ipath <IPATH>

OPTIONS:
    -h, --help             Print help information
    -i, --ipath <IPATH>    the input input path
    -s, --sep <SEP>        the column separator str [default: ,]
    -V, --version          Print version information
```

# Install

1. Clone this repo.

In this repo you then do:

```
cargo build -r
sudo cp target/release/dense_2_sparse /usr/bin/
```

You can of cause also use the target/release/dense_2_sparse program from the original point or copy it somewhere else.


## Testing

```
cargo build -r
target/release/dense_2_sparse -i testData -s "\t"
Rscript testData/Rtest.R
```

This output is expected:

```
Processing file "testData/DenseMatrix.csv"
I have detected 300 columns
100 columns 300 rows and 2693 data points read
sparse Matrix: 300 cell(s), 100 gene(s) and 100 entries written to path Ok("testData/DenseMatrix/filtered_feature_bc_matrix"); 
Attaching SeuratObject
Warning message:
In matrix(as.numeric(a), ncol = ncol(da)) : NAs introduced by coercion
[1] "OK"
```

## Spped

Using the 3,2G big GSE166895_postQC_mRNAraw_FL-FBM-CB.csv file from the Human Cell Atlast database (Bone -> "Blood and immune development in human fetal bone marrow and Down syndrome" )
takes 1min 46 sec on one core of an AMD Ryzen 5 3600X 6-Core Processor using Ubuntu 22.04.1 LTS with cernel Linux 5.15.0-58-generic and a magnetic disk as storage.

This is quite impressive - or?