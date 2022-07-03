<script lang="ts">
	import ProductCard from './ProductCard.svelte';
    import type { Product } from "src/model/product";
	let products: Product[] = JSON.parse(`
	[
		{
			"productId": 1,
			"manufacturer": "Johns-Jenkins",
			"sku": "p5z343vdS",
			"upc": "939581000000",
			"pricePerUnit": "497.45",
			"quantityOnHand": 9703,
			"productName": "sticky note"
		},
		{
			"productId": 2,
			"manufacturer": "Hessel, Schimmel and Feeney",
			"sku": "i7v300kmx",
			"upc": "740979000000",
			"pricePerUnit": "282.29",
			"quantityOnHand": 9217,
			"productName": "leg warmers"
		},
		{
			"productId": 3,
			"manufacturer": "Swaniawski, Bartoletti and Bruen",
			"sku": "q0L657ys7",
			"upc": "111730000000",
			"pricePerUnit": "436.26",
			"quantityOnHand": 5905,
			"productName": "lamp shade"
		},
		{
			"productId": 4,
			"manufacturer": "Runolfsdottir, Littel and Dicki",
			"sku": "x78426lq1",
			"upc": "93986215015",
			"pricePerUnit": "537.90",
			"quantityOnHand": 2642,
			"productName": "flowers"
		},
		{
			"productId": 5,
			"manufacturer": "Kuhn, Cronin and Spencer",
			"sku": "r4X793mdR",
			"upc": "260149000000",
			"pricePerUnit": "112.10",
			"quantityOnHand": 6144,
			"productName": "clamp"
		}
	]`);
</script>

<h1>Products</h1>

{#each products as product}
	<ProductCard product={product}/>
{/each}
