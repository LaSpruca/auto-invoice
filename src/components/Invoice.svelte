<script>
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
    theme,
    bankAccount,
    email,
    invoiceNumber,
    phoneNumber,
    payableTo
  } from "../stores";

  import moment from 'moment';
</script>

<div class="invoice">
  <div class="header">
    <div class="invoice-info">
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
</div>


<style lang="scss">
  .invoice {
    background: white;
    color: black;
    padding: 1rem;
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
</style>
