<script lang="ts">
  import Invoice from "../components/Invoice.svelte";
  import Fab, {Label, Icon} from "@smui/fab";

  function openPrintWindow() {
    if (typeof window !== 'undefined') {
      let printWindow = window.open('', 'PRINT', 'height=700,width=800');
      printWindow.document.write("<!DOCTYPE html><head>");
      printWindow.document.write(document.head.innerHTML);
      printWindow.document.write("</head><body><script>print();</" + "script>");
      printWindow.document.write(document.getElementById("content").innerHTML);
      printWindow.document.write("</body>");
    }
  }
</script>

<svelte:head>
  <title>Auto Invoice | Invoice</title>
</svelte:head>

<Fab on:click={() => {openPrintWindow()}} extended class="fab" style="position:fixed;bottom:1rem;right:1rem;">
  <Icon class="material-icons">print</Icon>
  <Label>Print</Label>
</Fab>

<div class="invoice" id="content">
  <Invoice/>
</div>

<style lang="scss">
  .invoice {
    padding: 0 10px 10px 10px;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    flex: 0 1 210mm;

    :global(.invoice) {
      width: 210mm;
      min-height: 297mm;
    }
  }

  @media print {
    .invoice {
      padding: 0;
      margin: 0;
    }
  }
</style>
