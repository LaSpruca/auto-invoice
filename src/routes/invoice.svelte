<script>
  import Invoice from "../components/Invoice.svelte";
</script>

<svelte:head>
  <title>Auto Invoice | Invoice</title>
</svelte:head>

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
      flex: 0 1 210mm;
      min-height: 297mm;
    }
  }
</style>
