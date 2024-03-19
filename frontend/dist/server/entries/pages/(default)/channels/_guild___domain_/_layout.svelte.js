import { c as create_ssr_component, v as validate_component } from "../../../../../chunks/ssr.js";
const GuildSkeleton = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<div class="flex-none flex flex-col" data-svelte-h="svelte-1pvbncp"><div class="flex-auto bg-green-50 mt-0 mb-2 rounded-md overflow-y-auto"></div> <ul class="flex-none menu menu-horizontal bg-base-300 rounded-md"><li class="mx-1"><button><i class="fa-solid fa-signal"></i></button></li> <li class="mx-1"><button><i class="fa-solid fa-volume-high"></i></button></li> <li class="mx-1"><button><i class="fa-solid fa-microphone"></i></button></li> <li class="mx-1"><button><i class="fa-solid fa-phone-slash"></i></button></li></ul></div> <div class="flex-auto flex">${slots.default ? slots.default({}) : ``}</div> <div class="flex-none" data-svelte-h="svelte-45g9op"><ul class="flex-none menu menu-horizontal bg-base-300 rounded-md"><li class="mx-1"><button><i class="fa-solid fa-signal"></i></button></li> <li class="mx-1"><button><i class="fa-solid fa-volume-high"></i></button></li> <li class="mx-1"><button><i class="fa-solid fa-microphone"></i></button></li> <li class="mx-1"><button><i class="fa-solid fa-phone-slash"></i></button></li></ul></div>`;
});
const Layout = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `${validate_component(GuildSkeleton, "GuildSkeleton").$$render($$result, {}, {}, {
    default: () => {
      return `${slots.default ? slots.default({}) : ``}`;
    }
  })}`;
});
export {
  Layout as default
};
