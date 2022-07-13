<script lang="ts">
    import type { Product } from "src/model/product";
    export let product: Product;
</script>

<div class="flex-col container p-4 m-1 bg-orange-200 drop-shadow rounded hover:bg-orange-100">
    <h2 class="text-center capitalize">{product.productName}</h2>
    <div class="img-placeholder justify-center"></div>
    <p class="text-right">${product.pricePerUnit}</p>
</div>

<style>
    .img-placeholder {
        margin: auto;
        margin-top: 10px;
        background-color: azure;
        height: 100px;
        width: 100px;
    }
</style>