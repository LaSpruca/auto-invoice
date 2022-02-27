import { writable } from "svelte/store";
import type { Writable} from "svelte/store";
import type { User } from "firebase/auth";

const user: Writable<User | null> = writable(null);

export {user};