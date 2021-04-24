<script lang="ts">
  import {
    items,
    dateDue,
    submittedDate,
    clientCompanyAddress2,
    clientCompanyAddress1,
    clientCompanyName,
    clientName,
    name,
    address1,
    address2,
    bankAccount,
    email,
    invoiceNumber,
    phoneNumber,
    payableTo,
    currency,
    adjustments,
    gst
  } from "../stores";

  function dp2(num: number) {
    return (Math.round(num * 100) / 100).toFixed(2)
  }

  import moment from 'moment';

  let subtotal = 0;
  let gstVal = 0;
  let total = 0;


  items.subscribe(value => {
    subtotal = value.map(e =>
      (!parseFloat(e.quantity) || !parseFloat(e.unitPrice)) ? 0 : parseFloat(e.quantity) * parseFloat(e.unitPrice)
    ).reduce((coll, val) => coll + val, 0);
    gstVal = subtotal * $gst;
    total = subtotal + gstVal + parseFloat($adjustments as string);
  });

  gst.subscribe(value => {
    gstVal = subtotal * value;
    total = subtotal + gstVal + parseFloat($adjustments as string);
  });

  adjustments.subscribe(value => {
    total = subtotal + gstVal + parseFloat(value as string);
  });
</script>

<div class="invoice">
  <div class="header">
    <div class="invoice-dates">
      <h1>Invoice</h1>
      <h3>Submitted: <span>{moment($submittedDate).format("dddd, MMMM Do YYYY")}</span></h3>
      <h3>Due: <span>{moment($dateDue).format("dddd, MMMM Do YYYY")}</span></h3>
    </div>

    <div class="company-info">
      <h2>{$name}</h2>
      <h3>{$address1}</h3>
      <h3>{$address2}</h3>
      <h4>{$email} | {$phoneNumber}</h4>
    </div>
  </div>
  <hr/>
  <div class="invoice-info">
    <div class="invoice-for">
      <h2>Invoice For</h2>
      <h3>{$clientName}</h3>
      <h4 class="grey">{$clientCompanyName}</h4>
      <div class="address">
        <h4>{$clientCompanyAddress1}</h4>
        <h4>{$clientCompanyAddress2}</h4>
      </div>
    </div>
    <div class="stack">
      <div class="invoice-number">
        <h2>Invoice Number</h2>
        <h3>{$invoiceNumber}</h3>
      </div>
      <div class="payment-info">
        <h2>Payable to</h2>
        <h3>{$payableTo}</h3>
        <h3>{$bankAccount}</h3>
      </div>
    </div>
  </div>
  <hr>
  <table class="expenses-table">
    <tr class="table-header">
      <th><span>Description</span></th>
      <th><span>Quantity</span></th>
      <th><span>Unit Price</span></th>
      <th><span>Total Price</span></th>
    </tr>
    {#each $items as it}
      {#if it.description !== ""}
        <tr>
          <td>{it.description}</td>
          <td>{it.quantity}</td>
          <td>{$currency} {!parseFloat(it.unitPrice) ? "0.00" : dp2(parseFloat(it.unitPrice))}</td>
          <td>{$currency} {dp2(it.unitPrice * it.quantity)}</td>
        </tr>
      {/if}
    {/each}
  </table>

  <hr/>

  <div class="totals">
    <h2>Subtotal: <span>{$currency}{dp2(subtotal)}</span></h2>
    <h3>GST: <span>{$currency}{dp2(gstVal)}</span></h3>
    <h3>Adjustments: <span>{$currency}{dp2(parseFloat($adjustments))}</span></h3>
    <h1>Total: <span>{$currency}{dp2(total)}</span></h1>
  </div>
</div>


<style lang="scss">
  .invoice {
    background: white;
    color: black;
    padding: 1rem;
    max-width: 210mm;
  }

  .header {
    display: flex;
    justify-content: space-between;

    h3 {
      span {
        font-weight: normal;
      }
    }

    .company-info {
      text-align: right;

      h3, h4 {
        font-weight: normal;
      }
    }
  }

  hr {
    margin: 1rem 0;
  }

  h4 {
    font-weight: normal;
  }

  .invoice-info {
    display: flex;
    justify-content: space-between;

    .grey {
      color: darkgrey;
    }

    .address {
      padding: 0.5rem 0;
    }

    .stack {
      display: flex;
      flex-direction: column-reverse;
      text-align: right;

      div {
        padding: 0.5rem 0;

        h3 {
          font-weight: normal;
        }
      }
    }
  }

  .expenses-table {
    width: 100%;

    th span {
      border-bottom: solid 3px grey;
      margin: 0 0.5rem 0 1.5rem;
    }

    th:nth-child(1) {
      text-align: left;
      width: 50%;
    }

    tr {
      border: none;
    }

    tr td {
      text-align: center;

      &:nth-child(1) {
        text-align: left;
      }
    }

    tr:nth-child(2n+1):not(.table-header) {
      background: #eee;
    }
  }

  .totals {
    text-align: right;

    h2, h3 {
      font-weight: normal;

      span {
        font-weight: bold;
      }
    }

    h1 {
      padding-top: 1rem;
    }
  }
</style>
