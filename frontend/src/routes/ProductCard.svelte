<script lang="ts">
    import type { Product } from "src/model/product";
    export let product: Product;
</script>

<div class="flex p-4 m-1 bg-orange-200">
    <h2>{product.productName}</h2>

    <p class="">${product.pricePerUnit}</p>
</div>
