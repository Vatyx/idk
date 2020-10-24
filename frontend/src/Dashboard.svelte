<script lang="typescript">
	import * as firebase from "firebase/app";
	var user = firebase.auth().currentUser;
	let data: any | undefined = undefined;
	let idToken: string;
	firebase
		.auth()
		.currentUser.getIdToken(/* forceRefresh */ true)
		.then(function (idToken) {
			idToken = idToken;
			return fetch("https://api.idkcli.com/user", {
				method: "post",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
					Authorization: `Bearer ${idToken}`,
				},
				body: JSON.stringify({
					name: user.displayName,
					email: user.email,
				}),
			});
		})
		.then((response) => response.json())
		.then((jsonResponse) => {
			data = jsonResponse;
			const urlParams = new URLSearchParams(window.location.search);
			if (urlParams.get('ref') === "cli") {
				return fetch(`http://localhost:8765/auth?token=${idToken}`, {
					method: "get",
				})
			};
		})
		.catch(function (error) {
			console.error('woops: ', error)
		});
</script>

<main>
	{#if data}
		<p>{JSON.stringify(data)}</p>
	{:else}
		<p>loading yo</p>
	{/if}
</main>
