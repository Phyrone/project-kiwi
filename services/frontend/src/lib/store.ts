import { persisted } from 'svelte-persisted-store';


export const menu_open = persisted<boolean>("preference.menu.open",false,{
	storage: "session",
	syncTabs: true
});