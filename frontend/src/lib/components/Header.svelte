<script lang="ts">
    import { user } from "../stores";
    import { logout } from "../authz";
    import { page } from "$app/stores";

    $: console.log($user.profile);
</script>

<header>
    <nav>
        <a href="/app/dashboard" class:active={$page.url.href == "/app/dashboard"}>Dashboard</a>
        <a href="/app/create" class:active={$page.url.href == "/app/create"}>Create Invoice</a>
        <a href="/app/previous" class:active={$page.url.href == "/app/previous"}>Previous Invoices</a>
    </nav>

    <div class="user">
        {#if $user}
            <img src={$user.picture} alt="Your avatar" />
        {/if}
        <button on:click={logout}>Logout</button>
    </div>
</header>

<style lang="scss">
    @import "../../colors";

    header {
        margin: 3rem;
        padding: 2rem;
        border-radius: 1rem;
        background-color: $background;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    nav {
        display: flex;
        gap: 1rem;

        a {
            color: white;
            text-decoration: none;
            font-size: 15pt;

            &.active {
                font-weight: bold;
            }
        }
    }

    .user {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 1rem;

        img {
            width: 3rem;
            height: 3rem;
            border-radius: 100%;
            outline: 2px white solid;
        }
    }
</style>