<script lang="ts">
	import Header from '$lib/components/Header.svelte';
</script>

<Header />

<div class="content">
	<slot />
</div>

<style lang="scss">
	@import '../../colors';

	.content {
		margin: 3rem;
		padding: 2rem;
		border-radius: 1rem;
		background-color: $background;
	}
</style>
