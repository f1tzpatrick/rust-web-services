<script lang="ts">
	import ProductCard from './ProductCard.svelte';
    import type { Product } from "src/model/product";

	import product_data from '$lib/data/products.json';
	import SideBar from '$lib/SideBar.svelte';
	let products: Product[] = product_data;
</script>

<SideBar />

<h1>Products</h1>

<div class="grid col-auto lg:grid-cols-5 md:grid-cols-3 sm:grid-cols-2 xs:grid-cols-1 gap-2 m-2">
	{#each products as product}
		<ProductCard product={product}/>
	{/each}
</div>