import createAuth0Client, { Auth0Client, type PopupLoginOptions } from "@auth0/auth0-spa-js";
import { user, userToken } from "./stores";
import { browser } from "$app/env";

let client = null;

function getClient(): Auth0Client {
	if (client == null) {
		throw new Error("Not running in browser, currently unsure how tf you achieved this");
	}

	return client;
}

async function createClient(): Promise<Auth0Client> {
	let authzClient = await createAuth0Client({
		client_id: import.meta.env.VITE_PUBLIC_AUTHZ_CLIENT_ID.toString(),
		domain: import.meta.env.VITE_PUBLIC_AUTHZ_DOMAIN.toString(),
		audience: import.meta.env.VITE_PUBLIC_AUTHZ_AUDIENCE.toString()
	})

	return authzClient;
}

export async function loginWithPopup(options: PopupLoginOptions) {
	try {
		let client = getClient();
		await client.loginWithPopup(options);

		user.set(await client.getUser());
		userToken.set(await client.getTokenSilently());
	} catch (e) {
		console.error(e);
	}
}

export async function logout() {
	try {
		await getClient().logout();
		user.set(null);
	} catch (ex) {
		console.error(ex);
	}
}

if (browser && client == null) {
	createClient().then((val) => {
		client = val;
		val.checkSession();
		if (val.isAuthenticated) {
			val.getUser().then(val => {
				user.set(val);
			});

			val.getTokenSilently().then(val => {
				userToken.set(val);
			});
		}
	});
}
