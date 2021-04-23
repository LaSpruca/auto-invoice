<script lang="ts">
  import TopAppBar, { Row, Section, Title } from "@smui/top-app-bar";
  import Drawer, { Content, Header, Scrim, Subtitle } from "@smui/drawer";
  import List, { Graphic, Item, Text } from "@smui/list";
  import IconButton from "@smui/icon-button";
  import { theme } from "../stores";
  import { stores } from "@sapper/app";

  const { page } = stores();

  let open = false;
</script>

<header>
  <Drawer bind:open fixed={false} variant="modal">
    <Header>
      <Title>Invoice Creator</Title>
      <Subtitle>brrr.</Subtitle>
    </Header>
    <Content>
      <List>
        <Item
          activated={$page.path === "/"}
          href="/"
          on:click={() => (open = false)}
        >
          <Graphic aria-hidden="true" class="material-icons">info</Graphic>
          <Text>Information</Text>
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
  <Scrim fixed={false} />

  <TopAppBar color="primary" variant="static">
    <Row>
      <Section>
        <IconButton class="material-icons" on:click={() => (open = !open)}
          >menu</IconButton
        >
        <Title>Invoice Creator</Title>
      </Section>
      <Section align="end" toolbar>
        <IconButton
          aria-label="Set theme"
          class="material-icons"
          on:click={() => ($theme = $theme === "light" ? "dark" : "light")}
        >
          {#if $theme === "dark"}light_mode{:else}dark_mode{/if}
        </IconButton>
      </Section>
    </Row>
  </TopAppBar>
</header>

<style lang="scss">
header {
    padding-bottom: 1rem;
}
</style>
