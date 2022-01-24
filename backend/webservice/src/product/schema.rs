table! {
    products (productId) {
        productId -> Integer,
        manufacturer -> Varchar,
        sku -> Varchar,
        upc -> Varchar,
        pricePerUnit -> Numeric,
        quantityOnHand -> Integer,
        productName -> Varchar,
    }
}
