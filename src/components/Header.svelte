<script lang="ts">
  import TopAppBar, {Row, Section, Title} from "@smui/top-app-bar";
  import Drawer, {Content, Header, Scrim, Subtitle} from "@smui/drawer";
  import List, {Graphic, Item, Text} from "@smui/list";
  import {Icon} from "@smui/common";
  import Svg from '@smui/common/Svg.svelte';
  import Tooltip, {Wrapper} from '@smui/tooltip';
  import IconButton from "@smui/icon-button";
  import {theme} from "../stores";
  import {stores} from "@sapper/app";
  import {
    mdiGithub,
  } from '@mdi/js';

  const {page} = stores();

  let open = false;
</script>

<header>
  <Drawer bind:open fixed={false} variant="modal">
    <Header>
      <Title>Invoice Creator</Title>
      <Subtitle>Select a stage</Subtitle>
    </Header>
    <Content>
      <List>
        <Item
            activated={$page.path === "/"}
            href="/"
            on:click={() => (open = false)}
        >
          <Graphic aria-hidden="true" class="material-icons">info</Graphic>
          <Text>Your Information</Text>
        </Item>
        <Item
            activated={$page.path === "/invoice-creator"}
            href="/invoice-creator"
            on:click={() => (open = false)}
        >
          <Graphic aria-hidden="true" class="material-icons">edit</Graphic>
          <Text>Invoice Information</Text>
        </Item>
        <Item
            activated={$page.path === "/invoice"}
            href="/invoice"
            on:click={() => (open = false)}
        >
          <Graphic aria-hidden="true" class="material-icons">receipt</Graphic>
          <Text>Invoice</Text>
        </Item>
      </List>
    </Content>
  </Drawer>
  <Scrim fixed={false}/>

  <TopAppBar color="primary" variant="static">
    <Row>
      <Section>
        <IconButton class="material-icons" on:click={() => (open = !open)}
        >menu
        </IconButton
        >
        <Title>
          <span class="bolder">Invoice Creator</span> |
          {#if $page.path === "/"}Your info
          {:else if $page.path === "/invoice-creator"}Invoice Information
          {:else if $page.path === "/invoice"}Invoice
          {:else}Error
          {/if}
        </Title>
      </Section>
      <Section align="end" toolbar>
        <Wrapper>
          <IconButton
              aria-label="Set theme"
              class="material-icons"
              on:click={() => ($theme = $theme === "light" ? "dark" : "light")}
          >
            {#if $theme === "dark"}light_mode{:else}dark_mode{/if}
          </IconButton>
          <Tooltip>Change to {$theme = $theme === "light" ? "dark" : "light"} mode</Tooltip>
        </Wrapper>
        <Wrapper>
          <IconButton href="https://github.com/hperrin/svelte-material-ui">
            <Icon component={Svg} viewBox="0 0 24 24">
              <path fill="currentColor" d={mdiGithub}/>
            </Icon>
          </IconButton>
          <Tooltip>Github Repo</Tooltip>
        </Wrapper>
      </Section>
    </Row>
  </TopAppBar>
</header>

<style lang="scss">
  header {
    padding-bottom: 1rem;
  }

  .bolder {
    font-weight: bolder;
    padding-right: 5px;
  }
</style>
