
export class Product {
    productId: number;
    manufacturer: string;
    sku: string;
    upc: string;
    pricePerUnit: string;
    quantityOnHand: number;
    productName: string;

    constructor(
        productId: number,
        manufacturer: string,
        sku: string,
        upc: string,
        pricePerUnit: string,
        quantityOnHand: number,
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

