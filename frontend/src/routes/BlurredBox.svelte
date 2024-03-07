<script lang="ts">
	export let background: string = 'rgba(255, 255, 255, 0.2)';
</script>

<div class="box" style="--background: {background};">
	<slot />
</div>

<style>
	div.box {
		width: 100%;
		height: 100%;
		background: var(--background);
		box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
		backdrop-filter: blur(7px);
		-webkit-backdrop-filter: blur(7px);
		border-radius: 10px;
	}
</style>
