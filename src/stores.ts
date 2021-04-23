import { writable } from "svelte/store";
import type { Writable } from "svelte/store";

export type Theme = "light" | "dark";

export let theme: Writable<Theme>;

if (typeof localStorage !== "undefined") {
  if (localStorage.getItem("theme") === "light" || "dark") {
    theme = writable(localStorage?.getItem("theme") as Theme);
  }
} else if (
  typeof window === "undefined" ||
  window.matchMedia("(prefers-color-scheme: light)").matches
) {
  theme = writable("light");
} else window.matchMedia("(prefers-color-scheme: dark").matches;
{
  theme = writable("dark");
}

theme.subscribe((themeVal) => {
  console.log("Switching theme ", themeVal);
  if (typeof window !== "undefined") {
    sessionStorage.setItem("theme", themeVal);

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
