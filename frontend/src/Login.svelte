<script lang="typescript">
	import * as firebase from "firebase/app";
	import Dashboard from "./Dashboard.svelte"
	import "firebase/auth";
	var firebaseConfig = {
		apiKey: "AIzaSyCdhK626b4KKXPvzayC6o2INWFcpMBA8UY",
		authDomain: "idk-293422.firebaseapp.com",
		databaseURL: "https://idk-293422.firebaseio.com",
		projectId: "idk-293422",
		storageBucket: "idk-293422.appspot.com",
		messagingSenderId: "1086724907992",
		appId: "1:1086724907992:web:f3f3ef30e3b60e192708ad",
		measurementId: "G-B42HQ5YY8W",
	};
	firebase.initializeApp(firebaseConfig);
	var provider = new firebase.auth.GoogleAuthProvider();
	var user = firebase.auth().currentUser;
	let loggedIn: boolean = false;
	let loggedInLoading: boolean = true;
	firebase.auth().onAuthStateChanged(function (user) {
		loggedInLoading = false;
		if (user) {
			loggedIn = true;
		}
	});
	const login = () => {
		firebase.auth().signInWithRedirect(provider);
	};
	const logout = () => {
		firebase
			.auth()
			.signOut()
			.then(() => window.location.reload());
	};
</script>

<main>
	{#if loggedInLoading}
			<div class="window" style="margin: 32px; width: 250px">
				<div class="title-bar">
					<div class="title-bar-text">Loading Screen</div>

					<div class="title-bar-controls">
						<button aria-label="Minimize" />
						<button aria-label="Maximize" />
						<button aria-label="Close" />
					</div>
				</div>
				<div class="window-body">
					<p>Your account is loading.</p>
				</div>
			</div>
	{:else}
		{#if loggedIn}
			<Dashboard/>
		{/if}
		{#if !loggedIn}
			<div class="window" style="margin: 32px; width: 250px">
				<div class="title-bar">
					<div class="title-bar-text">Login Screen</div>

					<div class="title-bar-controls">
						<button aria-label="Minimize" />
						<button aria-label="Maximize" />
						<button aria-label="Close" />
					</div>
				</div>
				<div class="window-body">
					<p>Would you like to login?</p>
					<section
						class="field-row"
						style="justify-content: flex-end">
						<button on:click={login}>Login</button>
					</section>
				</div>
			</div>
		{/if}
	{/if}
</main>
