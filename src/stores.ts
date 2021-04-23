import { writable } from "svelte/store";
import type { Writable } from "svelte/store";

export type Stored = {
  name: string;
  address1: string;
  address2: string;
  phoneNumber: string;
  email: string;
  invoiceNumber: number;
  bankAccount: string;
  payableTo: string;
};

export type Theme = "light" | "dark";

// Theme shit
export let theme: Writable<Theme>;

// Try load theme from session
if (typeof localStorage !== "undefined") {
  if (localStorage.getItem("theme") === "light" || "dark") {
    theme = writable(localStorage?.getItem("theme") as Theme);
  }
} else if (
  // Do they use light theme
  typeof window === "undefined" ||
  window.matchMedia("(prefers-color-scheme: light)").matches
) {
  theme = writable("light");
} else {
  // Default to dark theme
  theme = writable("dark");
}

// When them theme is changed, update shit
theme.subscribe((themeVal) => {
  console.log("Switching theme ", themeVal);
  if (typeof window !== "undefined") {
    localStorage.setItem("theme", themeVal);

    let themeLink = document.head.querySelector("#theme");

    if (!themeLink) {
      themeLink = document.createElement("link");
      // @ts-ignore
      themeLink.rel = "stylesheet";
      themeLink.id = "theme";
    }

    // @ts-ignore
    themeLink.href = `client/smui${themeVal == "light" ? "" : "-dark"}.css`;
    document.head
      .querySelector('link[href="client/smui-dark.css"]')
      .insertAdjacentElement("afterend", themeLink);
  }
});
const sessionStore: Writable<Stored> =
  typeof localStorage !== "undefined"
    ? writable(JSON.parse(localStorage.getItem("info")))
    : writable({});

let sessionStoreVal: Stored;

sessionStore.subscribe((val) => {
  sessionStoreVal = val;
  if (typeof localStorage !== "undefined") {
    localStorage.setItem("info", JSON.stringify(val));
  }
});

// All the stores
export const name: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.name ? sessionStoreVal.name : "" : ""
);
name.subscribe((val) => sessionStore.set({ ...sessionStoreVal, name: val }));

export const address1: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.address1 ? sessionStoreVal.address1 : "" : ""
);
address1.subscribe((val) =>
  sessionStore.set({ ...sessionStoreVal, address1: val })
);

export const address2: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.address2 ? sessionStoreVal.address2 : "" : ""
);
address2.subscribe((val) =>
  sessionStore.set({ ...sessionStoreVal, address2: val })
);

export const phoneNumber: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.phoneNumber ? sessionStoreVal.phoneNumber : "" : ""
);
phoneNumber.subscribe((val) =>
  sessionStore.set({ ...sessionStoreVal, phoneNumber: val })
);

export const email: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.email ? sessionStoreVal.email : "" : ""
);
email.subscribe((val) => sessionStore.set({ ...sessionStoreVal, email: val }));

export const invoiceNumber: Writable<number> = writable(
  sessionStoreVal ? sessionStoreVal.invoiceNumber ? sessionStoreVal.invoiceNumber : 0 : 0
);
invoiceNumber.subscribe((val) =>
  sessionStore.set({ ...sessionStoreVal, invoiceNumber: val })
);

export const bankAccount: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.bankAccount ? sessionStoreVal.bankAccount : "" : ""
);
bankAccount.subscribe((val) =>
  sessionStore.set({ ...sessionStoreVal, bankAccount: val })
);
export const payableTo: Writable<string> = writable(
  sessionStoreVal ? sessionStoreVal.payableTo ? sessionStoreVal.payableTo : "" : ""
);
payableTo.subscribe((val) =>
  sessionStore.set({ ...sessionStoreVal, payableTo: val })
);
