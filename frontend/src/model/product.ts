
export class Product {
    productId: string;
    manufacturer: string;
    sku: string;
    upc: string;
    pricePerUnit: string;
    quantityOnHand: string;
    productName: string;

    constructor(
        productId: string,
        manufacturer: string,
        sku: string,
        upc: string,
        pricePerUnit: string,
        quantityOnHand: string,
        productName: string,
    ) {
        this.productId = productId;
        this.manufacturer = manufacturer;
        this.sku = sku;
        this.upc = upc;
        this.pricePerUnit = pricePerUnit;
        this.quantityOnHand = quantityOnHand;
        this.productName = productName;
    }
}
