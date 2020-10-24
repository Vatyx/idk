<script lang="ts">
	import { createClient } from '@supabase/supabase-js'
	const SUPABASE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYwMzUwMjQ5MSwiZXhwIjoxOTE5MDc4NDkxfQ.z94jL3kJSO8awkd3h_yvdhsX5lsgmStFmQZShRBzoi4'
	const SUPABASE_URL = "https://bhlpgnzrnvrchycujafn.supabase.co"
	const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);
	export let name: string;
	let value: string;
	supabase
		.from('Prompt')
		.select('*').then(r => {
		value = r.body[0].content;
		})

	const saveContent = () => {
		supabase
		.from('Prompt')
		.update({ content: value })
		.eq('id', 1).then(r => console.log(r.body))
	}
</script>

<main>
	<textarea cols="8" row="8" bind:value></textarea>
	<button on:click={saveContent}>save</button>
</main>

<style>
	textarea {
		height: 200px;
		width: 800px;
	}

	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
