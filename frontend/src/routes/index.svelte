<script lang="ts">
	import ProductCard from './ProductCard.svelte';
    import type { Product } from "src/model/product";

	import product_data from '$lib/data/products.json';
	let products: Product[] = product_data;
</script>

<h1>Products</h1>

{#each products as product}
	<ProductCard product={product}/>
{/each}
