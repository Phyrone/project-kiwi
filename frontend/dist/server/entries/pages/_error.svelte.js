import { c as create_ssr_component, a as subscribe, v as validate_component, f as escape } from "../../chunks/ssr.js";
import { A as AppSkeleton } from "../../chunks/AppSkeleton.js";
import { p as page } from "../../chunks/stores.js";
const Error = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $page, $$unsubscribe_page;
  $$unsubscribe_page = subscribe(page, (value) => $page = value);
  $$unsubscribe_page();
  return `${validate_component(AppSkeleton, "AppSkeleton").$$render($$result, {}, {}, {
    default: () => {
      return `<div class="flex-auto grid place-items-center"><div class="w-32"><h1>${escape($page.status)}</h1> <h2>${escape($page.error.message)}</h2></div></div>`;
    }
  })}`;
});
export {
  Error as default
};
