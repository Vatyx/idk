<script lang="typescript">
	import * as firebase from "firebase/app";
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
	firebase.auth().onAuthStateChanged(function (user) {
		if (user) {
			loggedIn = true;
		}
	});
	const login = () => {
		firebase.auth().signInWithRedirect(provider);
	};
	const logout = () => {
		firebase.auth().signOut().then(() => window.location.reload());
	};
</script>

<main>
	{#if loggedIn}
	Congrats buddy, you are logged in.
	<button on:click={logout}> logout </button>
	{/if}
	{#if !loggedIn}
	Bruh, you should login.
	<button on:click={login}> login </button>
	{/if}
</main>
