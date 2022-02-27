import { initializeApp } from 'firebase/app';
import { getAuth, GoogleAuthProvider, onAuthStateChanged, signInWithPopup } from 'firebase/auth';
import type { UserCredential } from 'firebase/auth';
import { getAnalytics } from 'firebase/analytics';
import { browser } from '$app/env';
import { user as userStore } from "./stores";

let google: () => Promise<UserCredential> = async () => { throw new Error("Not in the browser, how is this function even being run?") };

if (browser) {
	const firebaseConfig = {
		apiKey: 'AIzaSyB3ta94xiNRTsXkpnHzInv3U1hrZuAxQoA',
		authDomain: 'auto-invoice-67aff.firebaseapp.com',
		projectId: 'auto-invoice-67aff',
		storageBucket: 'auto-invoice-67aff.appspot.com',
		messagingSenderId: '693854333513',
		appId: '1:693854333513:web:bba919917b95c91e9dad98',
		measurementId: 'G-Y20K61DX47'
	};

	const app = initializeApp(firebaseConfig);
	const analytics = getAnalytics(app);
	const auth = getAuth(app);
	const googleAuth = new GoogleAuthProvider();

    google = async () => await signInWithPopup(auth, googleAuth);

    onAuthStateChanged(auth, (user) => {
        userStore.set(user);
    });
}

export { google as signInWithGoogle };