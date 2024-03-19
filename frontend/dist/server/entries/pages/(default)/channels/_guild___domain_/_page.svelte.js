import { c as create_ssr_component } from "../../../../../chunks/ssr.js";
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { data } = $$props;
  if ($$props.data === void 0 && $$bindings.data && data !== void 0)
    $$bindings.data(data);
  return `<div class="mx-auto h-full bg-green-50 rounded-md w-full max-w-screen-md" data-svelte-h="svelte-1ew5kbg">ABC</div>`;
});
export {
  Page as default
};
