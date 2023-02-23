library(Seurat)

da = read.delim('testData/DenseMatrix.csv',sep="\t", row.names=1)
a = as.matrix(da)
a = matrix(as.numeric(a), ncol=ncol(da))
sp = Matrix::Matrix( a, sparse=TRUE) 

rownames(sp) = rownames(da)

cmd =  "target/release/dense2sparse -i testData -sep=\"\\t\" -t yes"

print ( cmd )
system( cmd )

data = Read10X( "testData/DenseMatrix/filtered_feature_bc_matrix")

if ( length( all.equal( data, Matrix::t(sp)  ) == 1 ) ){
	print ("OK")
}else {
	print ("FAIL")
}
