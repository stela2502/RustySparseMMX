library(Seurat)
library(reshape2)
df = read.delim('testData/chimera/cell2.csv',sep=",", header=F)
mat = acast(df,V3 ~ V1,value.var='V5' ,fun.agg = max )
mat[which(is.na(mat))] = 0
cmd =  "target/release/chimera2sparse -i testData/chimera "
print( cmd )
system( cmd )

data = Read10X( "testData/chimera/cell2/filtered_feature_bc_matrix")

a = as.matrix(df)
a = matrix(as.numeric(a), ncol=ncol(df))
sp = Matrix::Matrix( a, sparse=TRUE) 

rownames(sp) = rownames(df)

if ( length( all.equal( data, Matrix::Matrix( sp ) ) == 1 ) ){
	print ("OK")
}else {
	print ("FAIL")
}
