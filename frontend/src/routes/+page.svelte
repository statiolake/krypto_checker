<script lang="ts">
	import type { FormEventHandler } from 'svelte/elements';
	import BlurredBox from './BlurredBox.svelte';
	import NumberCard from './NumberCard.svelte';
	import { computeFormula, generateQuiz, parseFormula } from './game';

	let inputted: boolean = false;
	let computed: string = '?';
	let parsed: boolean = false;

	const { cards, target } = generateQuiz();

	const onInput: FormEventHandler<HTMLInputElement> = (e) => {
		const value = e.currentTarget.value;
		inputted = value !== '';

		parsed = false;
		computed = '?';
		const formula = parseFormula(value);
		console.log('got formula:', formula);
		if (formula != null) {
			const value = computeFormula(formula)?.toString();
			if (value != null) {
				computed = value;
				parsed = true;
			}
		}
	};
</script>

<svelte:head>
	<title>Krypto</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1 class="title">Krypto</h1>

	<div class="original-cards-container">
		{#each cards as card}
			<NumberCard width="12%">{card}</NumberCard>
		{/each}
		<span class="operator">=</span>
		<NumberCard width="12%">{target}</NumberCard>
		<span class="operator">?</span>
	</div>

	<div class="formula-wrapper">
		<BlurredBox>
			<input class="formula" placeholder="Answer?" on:input={onInput} />
		</BlurredBox>
	</div>

	<div class="computed {inputted ? 'visible' : ''} {parsed ? 'parsed' : ''}">
		<NumberCard width="6rem">{computed}</NumberCard>
	</div>
</section>

<style>
	.title {
		font-size: 60pt;
	}

	.original-cards-container {
		width: 100%;
		font-size: 4em;
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin: 2rem 0;
		transition: all 0.1s linear;
	}

	/* .original-cards-container.shrinked {
		font-size: 1em;
		width: 30%;
	} */

	.formula-wrapper {
		position: absolute;
		top: 70%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 50%;
	}

	.formula {
		/* reset default style */
		margin: 0;
		padding: 0;
		background: none;
		border: none;
		border-radius: 0;
		outline: none;
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;

		margin: 1rem;
		font-size: 3rem;
	}

	.computed {
		font-size: 3rem;
		color: #999;
		position: absolute;
		top: 70%;
		right: 10%;
		transform: translateY(-50%);
		opacity: 0;
		transition: all 0.1s ease-in-out;
	}

	.computed.visible {
		opacity: 1;
	}

	.computed.visible.parsed {
		color: #fff;
	}
</style>
