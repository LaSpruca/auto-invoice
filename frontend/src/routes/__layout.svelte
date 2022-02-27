<script lang="ts">
	import '../global.scss';
	import { user } from '$lib/stores';
	import Index from './index.svelte';
import { getPrivileged } from '$lib/api';
	
	$: {
		getPrivileged("/porn").then((res) => {
			console.log(res);
		}).catch(err => console.error(err));
		if ($user) {
			console.log($user.toJSON())
		}
	}

</script>

{#if user}
	<slot />
{:else}
	<Index />
{/if}
