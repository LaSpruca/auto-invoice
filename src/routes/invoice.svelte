<script>
  import Invoice from "../components/Invoice.svelte";
</script>

<div class="invoice">
  <Invoice/>
</div>


<style lang="scss">
  .invoice {
    padding: 0 10px 10px 10px;
    display: flex;
    justify-content: center;
    align-items: center;

    :global(.invoice) {
      min-height: 297mm;
    }
  }
</style>
