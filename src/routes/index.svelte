<script>
  import Textfield from '@smui/textfield';
  import Icon from '@smui/textfield/icon';
  import Fab, {Label} from "@smui/fab";

  import {
    name,
    address1,
    address2,
    phoneNumber,
    email,
    bankAccount,
    payableTo,
  } from "../stores";
</script>

<svelte:head>
  <title>Auto Invoice | Information</title>
</svelte:head>

<h1>Your Info</h1>

<div class="inputs">
  <Textfield variant="outlined" bind:value={$name} label="Name">
    <Icon class="material-icons" slot="leadingIcon">badge</Icon>
  </Textfield>

  <Textfield variant="outlined" bind:value={$phoneNumber} label="Phone Number">
    <Icon class="material-icons" slot="leadingIcon">phone</Icon>
  </Textfield>

  <Textfield variant="outlined" bind:value={$address1} label="Address Line 1">
    <Icon class="material-icons" slot="leadingIcon">business</Icon>
  </Textfield>

  <Textfield variant="outlined" bind:value={$address2} label="Address Line 2">
    <Icon class="material-icons" slot="leadingIcon">business</Icon>
  </Textfield>


  <Textfield variant="outlined" bind:value={$email} label="Email Address" type="email">
    <Icon class="material-icons" slot="leadingIcon">alternate_email</Icon>
  </Textfield>

  <Textfield variant="outlined" bind:value={$payableTo} label="Payable to">
    <Icon class="material-icons" slot="leadingIcon">person</Icon>
  </Textfield>

  <Textfield variant="outlined" bind:value={$bankAccount} label="Bank Account" type="bank-account">
    <Icon class="material-icons" slot="leadingIcon">account_balance</Icon>
  </Textfield>
</div>

<Fab expanded href="/invoice-creator" extended style="position: fixed; bottom: 1rem; right: 1rem">
  <Icon class="material-icons">navigate_next</Icon>
  <Label>Next</Label>
</Fab>

<style lang="scss">
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
</style>
