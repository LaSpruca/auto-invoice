<script lang="ts">
	import { user } from '$lib/stores';
    import { getPrivileged } from '$lib/api';
</script>

{#if $user}
    <h1>{$user.name}</h1>
{/if}

<style lang="scss">
    
</style>