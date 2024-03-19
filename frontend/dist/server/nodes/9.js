import * as universal from '../entries/pages/index.html/_page.ts.js';

export const index = 9;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/index.html/_page.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/index.html/+page.ts";
export const imports = ["_app/immutable/nodes/9.B9S5jq8p.js","_app/immutable/chunks/scheduler.DdtR2XwI.js","_app/immutable/chunks/index.B8Aw_p53.js","_app/immutable/chunks/entry.CNWT8fPr.js","_app/immutable/chunks/index.B9F4-EJT.js"];
export const stylesheets = [];
export const fonts = [];
