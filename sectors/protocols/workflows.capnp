@0xc7b7274275a0a8f9;

interface ProductRoutines {
    createProduct @0 (name: Text) -> (prod: Product);
}
# 每个interface可以当作是状态, 返回值则是进行的子状态
interface Product {
    createProductFeature @0 (name: Text) -> (prodFeat: ProductFeature);
}
interface ProductFeature {
}
