import { c as create_ssr_component } from "../../../chunks/ssr.js";
import { g as goto } from "../../../chunks/client.js";
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  goto("/", { replaceState: true });
  return `<h1 data-svelte-h="svelte-1odrn7i">Redirecting...</h1>`;
});
export {
  Page as default
};
