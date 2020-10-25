<script lang="typescript">
	import { Router, Link, Route } from "svelte-routing";
	import Prompt from "./Prompt.svelte"
	import Login from "./Login.svelte"
	export let url = "";
	console.log(url);
</script>

<Router url="{url}">
  <div>
    <Route path="/prompt" component="{Prompt}" />
    <Route path="/" component="{Login}" />
  </div>
</Router>

