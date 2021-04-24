<script lang="ts">
  import Textfield, {Prefix} from '@smui/textfield';
  import Icon from '@smui/textfield/icon';
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

<style lang="scss">
  #the-great-divide {
    display: grid;
    grid-template-columns: 50% 50%;

    .inputs {
      display: grid;
      grid-template-columns: auto auto;

      & > :global(label) {
        margin: 10px;
      }
    }

    h1 {
      text-align: center;
    }
  }

</style>
