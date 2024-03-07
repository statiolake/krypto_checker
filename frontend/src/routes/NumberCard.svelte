<script lang="ts">
	import BlurredBox from './BlurredBox.svelte';
	import WidthBasedBox from './WidthBasedBox.svelte';
	export let width: string;
</script>

<WidthBasedBox {width} height="130%">
	<BlurredBox background="rgba(255, 255, 255, 0.15)">
		<div class="number" style="--width: {width};">
			<slot />
		</div>
	</BlurredBox>
</WidthBasedBox>

<style>
	div.number {
		font-size: inherit;
		font-weight: bold;
		color: inherit;
		margin: 0;
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
	}
</style>
