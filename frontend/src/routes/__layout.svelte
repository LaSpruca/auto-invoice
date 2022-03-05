<script lang="ts">
	import '../global.scss';
	import { user, userToken } from '$lib/stores';
	import Index from './index.svelte';
	import { getPrivileged } from '$lib/api';

	$: {
		if ($userToken) {
			getPrivileged('/porn')
				.then((res) => {
					console.log(res);
				})
				.catch((err) => console.error(err));
		}
	}
</script>

{#if $user}
	<slot />
{:else}
	<Index />
{/if}
