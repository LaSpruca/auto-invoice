import { writable } from "svelte/store";
import type { Writable} from "svelte/store";
import type { User } from "@auth0/auth0-spa-js";

const user: Writable<User | null> = writable(null);
const userToken: Writable<string |  null> = writable(null);

export { user, userToken };