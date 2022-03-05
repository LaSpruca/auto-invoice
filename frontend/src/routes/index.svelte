<script lang="ts">
	import { user } from '$lib/stores';
	import { goto } from '$app/navigation';
	import { loginWithPopup } from '$lib/authz';

	$: if ($user) {
		goto('/app/dashboard');
	}
</script>

<div class="container">
	<div class="wrapper">
		<h1>Auto invoice</h1>
		<button on:click={loginWithPopup}>Login</button>
	</div>
</div>

<style lang="scss">
	@import '../colors';

	.container {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
		color: $primary;
	}

	.wrapper {
		animation: boc 0.2s ease-in-out 0.3s;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 1rem;
		padding: 5rem;
		border: 0 solid white;
		border-radius: 1rem;
		animation-fill-mode: forwards;
	}

	h1 {
		font-size: 20opt;
		animation: elvate 0.2s ease-in-out 0.3s;
		animation-fill-mode: forwards;
		transform: translateY(calc(2.5vh + 0.5rem));
	}

	button {
		background-color: $background-dark;
		border: none;
		border-radius: 1rem;
		padding: 1rem;
		color: $primary;
		font-size: 12.5pt;
		animation: unvaish 0.2s ease-in-out 0.3s;
		animation-fill-mode: forwards;
		transition: all 0.1s ease-in-out;
		opacity: 0;

		&:hover {
			cursor: pointer;
			transform: scale(1.5);
			color: $primary-lighter;
		}
	}

	@keyframes elvate {
		0% {
			transform: translateY(calc(2.5vh + 0.5rem));
		}
		100% {
			transform: translateY(0);
		}
	}

	@keyframes boc {
		0% {
			background: none;
		}
		0%,
		49% {
			border: 0 white solid;
		}
		100% {
			background-color: $background;
			border: 2px white solid;
		}
	}

	@keyframes unvaish {
		0%,
		50% {
			transform: translateY(calc(-1 * (1rem + 1vh)));
		}
		60% {
			opacity: 0;
		}
		100% {
			transform: translateY(0);
			opacity: 100%;
		}
	}
</style>
