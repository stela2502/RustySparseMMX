library(Seurat)
da = read.delim('testData/DenseMatrix.csv',sep="\t", row.names=1)

cmd =  "target/release/dense2sparse -i testData -sep=\"\\t\" "
print( cmd )
system( cmd )

data = Read10X( "testData/DenseMatrix/filtered_feature_bc_matrix")

a = as.matrix(da)
a = matrix(as.numeric(a), ncol=ncol(da))
sp = Matrix::Matrix( a, sparse=TRUE) 

rownames(sp) = rownames(da)

if ( length( all.equal( data, Matrix::Matrix( sp ) ) == 1 ) ){
	print ("OK")
}else {
	print ("FAIL")
}
