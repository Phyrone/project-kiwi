import { c as create_ssr_component, g as each, b as add_attribute, f as escape, v as validate_component } from "./ssr.js";
const button_count = 100;
const Sidebar = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  const buttons = Array.from({ length: button_count }, (_, i) => i + 1);
  return `<div class="flex-none my-2 mx-2 py-2 px-1 rounded-md bg-base-300 overflow-y-auto overflow-x-hidden overscroll-auto flex flex-col">${each(buttons, (button) => {
    return `<a class="btn btn-circle btn-neutral rounded-3xl delay-0 duration-150 will-change-transform m-1 transition-all hover:rounded-lg ease-in-out "${add_attribute("href", "/channels/_:" + button + "/", 0)}>${escape(button)}</a>`;
  })}</div>`;
});
const NotificationBell = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { count = 0 } = $$props;
  let show_indicator = false;
  if ($$props.count === void 0 && $$bindings.count && count !== void 0)
    $$bindings.count(count);
  show_indicator = count > 0;
  return `<button class="${["btn mx-2", show_indicator ? "indicator" : ""].join(" ").trim()}"><i class="fa-solid fa-bell"></i> ${show_indicator ? `<span class="indicator-item indicator-bottom badge badge-secondary text-xs">${escape(count > 99 ? "99+" : count)}</span>` : ``}</button>`;
});
const TopNavSearchBar = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let search_query = "";
  return `<label class="flex-auto flex items-center input input-bordered"><i class="opacity-70 fa-solid fa-magnifying-glass"></i> <input type="text" placeholder="Search" class="w-full border-0"${add_attribute("value", search_query, 0)}>  </label>`;
});
const TopNavbar = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<nav class="bg-base-300 my-2 flex px-2 py-1 rounded-md align-middle"><div data-svelte-h="svelte-24zh3l"><a class="btn btn-ghost" href="/"><i class="fa-solid fa-compass"></i></a></div> <div class="flex-auto" data-svelte-h="svelte-z75huh"></div> ${validate_component(TopNavSearchBar, "TopNavSearchBar").$$render($$result, {}, {}, {})} <div class="flex-1" data-svelte-h="svelte-s02ffn"></div> <div>${validate_component(NotificationBell, "NotificationBell").$$render($$result, { count: 2e3 }, {}, {})} <a class="btn btn-ghost" href="/profile/" data-svelte-h="svelte-1b4ny6f"><i class="fa-solid fa-user"></i></a></div></nav>`;
});
const AppSkeleton = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<div class="flex h-screen overflow-hidden">${validate_component(Sidebar, "Sidebar").$$render($$result, {}, {}, {})} <div class="flex-auto mr-2 ml-0 flex flex-col">${validate_component(TopNavbar, "TopNavbar").$$render($$result, {}, {}, {})} <div class="flex-auto flex mb-2">${slots.default ? slots.default({}) : ``}</div></div></div>`;
});
export {
  AppSkeleton as A
};
