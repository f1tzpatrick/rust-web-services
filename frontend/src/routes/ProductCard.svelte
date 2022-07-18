<script lang="ts">
    import type { Product } from "src/model/product";
    export let product: Product;
</script>

<div class="flex flex-col container p-4 m-1 bg-green-800 drop-shadow-lg rounded-3xl transform hover:scale-105 hover:bg-green-300 duration-200">
    <h2 class="text-center capitalize">{product.productName}</h2>
    <div class="justify-center m-auto p-5 h-5 w-5 bg-white"></div>
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