<script lang="ts">
	export let width: string;
	export let height: string;
</script>

<div class="wrapper" style="--width: {width};">
	<div class="height-occupier" style="--height: {height};">
		<div class="container">
			<slot />
		</div>
	</div>
</div>

<style>
	.wrapper {
		width: var(--width);
	}

	.height-occupier {
		padding-top: var(--height);
		position: relative;
	}

	.container {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}
</style>
