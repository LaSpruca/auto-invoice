<script lang="ts">
  import Textfield, {Prefix} from '@smui/textfield';
  import Icon from '@smui/textfield/icon';
  import Fab, {Label} from "@smui/fab";
  // @ts-ignore
  import DatePicker from "svelte-calendar";

  import {
    clientName,
    clientCompanyName,
    clientCompanyAddress1,
    clientCompanyAddress2,
    invoiceNumber,
    submittedDate,
    dateDue,
    currency,
    gst,
    adjustments
  } from '../stores';
  import InvoiceItems from "../components/InvoiceItems.svelte";
  import Invoice from "../components/Invoice.svelte";

  submittedDate.subscribe((val) => console.log(val));
</script>

<svelte:head>
  <title>Auto Invoice | Invoice Creator</title>
</svelte:head>


<div id="the-great-divide">
  <div class="left">
    <h1>Client info</h1>
    <div class="inputs">
      <Textfield variant="outlined" bind:value={$clientName} label="Name" type="text">
        <Icon class="material-icons" slot="leadingIcon">person</Icon>
      </Textfield>

      <Textfield variant="outlined" bind:value={$clientCompanyName} label="Company Name" type="text">
        <Icon class="material-icons" slot="leadingIcon">badge</Icon>
      </Textfield>

      <Textfield variant="outlined" bind:value={$clientCompanyAddress1} label="Address Line 1" type="text">
        <Icon class="material-icons" slot="leadingIcon">business</Icon>
      </Textfield>

      <Textfield variant="outlined" bind:value={$clientCompanyAddress2} label="Address Line 2" type="text">
        <Icon class="material-icons" slot="leadingIcon">business</Icon>
      </Textfield>
    </div>
    <h1>Invoice Info</h1>
    <div class="inputs">
      <div class="dp">
        <h4>Date submitted</h4>
        <DatePicker
            bind:selected={$submittedDate}
        />
      </div>

      <div class="dp">
        <h4>Date due</h4>
        <DatePicker
            bind:selected={$dateDue}
        />
      </div>

      <Textfield variant="outlined" bind:value={$invoiceNumber} label="Invoice Number" type="number">
        <Icon class="material-icons" slot="leadingIcon">tag</Icon>
      </Textfield>

      <Textfield variant="outlined" bind:value={$currency} label="Currency" type="text"/>

      <Textfield variant="outlined" bind:value={$adjustments} label="Adjustments" type="float">
        <Icon class="material-icons" slot="leadingIcon">edit</Icon>
        <Prefix>{$currency}</Prefix>
      </Textfield>

      <Textfield variant="outlined" bind:value={$gst} label="GST" type="float">
        <Icon class="material-icons" slot="leadingIcon">tag</Icon>
      </Textfield>
    </div>
    <h1>Items</h1>
    <div class="data">
      <InvoiceItems/>
    </div>
  </div>

  <div class="right">
    <Invoice/>
  </div>
</div>

<Fab expanded href="/invoice" extended style="position: fixed; bottom: 1rem; right: 1rem">
  <Icon class="material-icons">navigate_next</Icon>
  <Label>Next</Label>
</Fab>

<style lang="scss">
  #the-great-divide {
    display: flex;
    grid-template-columns: 50% 50%;
    justify-content: space-evenly;
    align-content: center;


    .inputs {
      display: grid;
      grid-template-columns: auto auto;
      width: 100%;

      & > :global(label) {
        margin: 10px;
      }
    }

    .right {
      width: max-content;
      flex: 0 1 210mm;
    }

    h1 {
      text-align: center;
    }
  }

</style>
