<script lang="ts">
  import type {Item} from "../stores";
  import {items, currency} from "../stores";
  import type {Writable} from "svelte/store";
  import {writable} from "svelte/store";
  import Textfield, {Prefix} from '@smui/textfield';
  import Icon from '@smui/textfield/icon';
  import IconButton from '@smui/icon-button';

  let data: Writable<any[]> = writable([]);

  let itemUpdate = false;

  items.subscribe(value => {
    itemUpdate = true;
    let temp = [];
    for (let a of value) {
      temp.push([a.description, a.quantity, a.unitPrice]);
    }
    data.set(temp);
  });

  data.subscribe(value => {
    if (itemUpdate) {
      itemUpdate = false;
      return;
    }
    let temp: Item[] = [];

    for (let a of value) {
      temp.push({description: a[0] as string, quantity: a[1], unitPrice: a[2]});
    }

    items.set(temp);
  });
</script>

<div id="items">
  {#each $data as item, itemIndex}
    <div class="item">
      {#each item as field, index}
        {#if index === 0}
          <Textfield bind:value={field} label="Description" type="text" class="description">
            <Icon class="material-icons" slot="leadingIcon">description</Icon>
          </Textfield>
        {:else if index === 1}
          <Textfield bind:value={field} label="Quantity" type="float">
            <Icon class="material-icons" slot="leadingIcon">tag</Icon>
          </Textfield>
        {:else}
          <Textfield bind:value={field} label="Unit Pice" type="float">
            <Prefix>{$currency}</Prefix>
          </Textfield>
        {/if}
      {/each}
      <IconButton class="material-icons" on:click={() => {
      let temp = $data;
      temp.splice(itemIndex, 1);
      data.set(temp);
    }}>delete
      </IconButton>
    </div>
  {/each}
</div>

<style lang="scss">
  #items {
    overflow-x: hidden;
    overflow-y: auto;
    height: 100%;
  }

  .item {
    display: flex;
    align-items: center;

    :global(label) {
      margin: 10px;

      &.description {
        flex: 0 70%;
      }
    }
  }
</style>
