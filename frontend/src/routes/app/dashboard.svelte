<script lang="ts">
	import { user } from '$lib/stores';
    import { getPrivileged } from '$lib/api';
</script>

{#if $user}
    <h1>{$user.name}</h1>
    <button on:click={_ => getPrivileged("/porn").then(console.log).catch(console.error)}>Send a request</button>
{/if}

<style lang="scss">
    
</style>