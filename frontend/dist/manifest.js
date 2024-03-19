export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		client: {"start":"_app/immutable/entry/start.C4GXYQfx.js","app":"_app/immutable/entry/app.gayRTw6P.js","imports":["_app/immutable/entry/start.C4GXYQfx.js","_app/immutable/chunks/entry.CNWT8fPr.js","_app/immutable/chunks/scheduler.DdtR2XwI.js","_app/immutable/chunks/index.B9F4-EJT.js","_app/immutable/entry/app.gayRTw6P.js","_app/immutable/chunks/scheduler.DdtR2XwI.js","_app/immutable/chunks/index.B8Aw_p53.js"],"stylesheets":[],"fonts":[],"uses_env_dynamic_public":false},
		nodes: [
			__memo(() => import('./server/nodes/0.js')),
			__memo(() => import('./server/nodes/1.js')),
			__memo(() => import('./server/nodes/2.js')),
			__memo(() => import('./server/nodes/3.js')),
			__memo(() => import('./server/nodes/5.js')),
			__memo(() => import('./server/nodes/6.js')),
			__memo(() => import('./server/nodes/7.js')),
			__memo(() => import('./server/nodes/8.js')),
			__memo(() => import('./server/nodes/9.js'))
		],
		routes: [
			{
				id: "/(default)",
				pattern: /^\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/api/v1/channels/@[channel]/messages",
				pattern: /^\/api\/v1\/channels\/@([^/]+?)\/messages\/?$/,
				params: [{"name":"channel","optional":false,"rest":false,"chained":false}],
				page: null,
				endpoint: __memo(() => import('./server/entries/endpoints/api/v1/channels/@_channel_/messages/_server.ts.js'))
			},
			{
				id: "/api/v1/media/create",
				pattern: /^\/api\/v1\/media\/create\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./server/entries/endpoints/api/v1/media/create/_server.ts.js'))
			},
			{
				id: "/api/v1/posts/@_:[id]",
				pattern: /^\/api\/v1\/posts\/@_:([^/]+?)\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: null,
				endpoint: __memo(() => import('./server/entries/endpoints/api/v1/posts/@___id_/_server.ts.js'))
			},
			{
				id: "/api/v1/posts/@[domain]:[id]",
				pattern: /^\/api\/v1\/posts\/@([^/]+?):([^/]+?)\/?$/,
				params: [{"name":"domain","optional":false,"rest":false,"chained":false},{"name":"id","optional":false,"rest":false,"chained":false}],
				page: null,
				endpoint: __memo(() => import('./server/entries/endpoints/api/v1/posts/@_domain___id_/_server.ts.js'))
			},
			{
				id: "/api/v1/posts/create",
				pattern: /^\/api\/v1\/posts\/create\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./server/entries/endpoints/api/v1/posts/create/_server.ts.js'))
			},
			{
				id: "/api/v1/users/self/login/userpass",
				pattern: /^\/api\/v1\/users\/self\/login\/userpass\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./server/entries/endpoints/api/v1/users/self/login/userpass/_server.ts.js'))
			},
			{
				id: "/(default)/channels/[guild]:[domain]",
				pattern: /^\/channels\/([^/]+?):([^/]+?)\/?$/,
				params: [{"name":"guild","optional":false,"rest":false,"chained":false},{"name":"domain","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,2,3,], errors: [1,,,], leaf: 5 },
				endpoint: null
			},
			{
				id: "/(default)/channels/[guild]:[domain]/[channel]",
				pattern: /^\/channels\/([^/]+?):([^/]+?)\/([^/]+?)\/?$/,
				params: [{"name":"guild","optional":false,"rest":false,"chained":false},{"name":"domain","optional":false,"rest":false,"chained":false},{"name":"channel","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,2,3,], errors: [1,,,], leaf: 6 },
				endpoint: null
			},
			{
				id: "/index.html",
				pattern: /^\/index\.html\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 8 },
				endpoint: null
			},
			{
				id: "/(default)/post/[post]:[domain]",
				pattern: /^\/post\/([^/]+?):([^/]+?)\/?$/,
				params: [{"name":"post","optional":false,"rest":false,"chained":false},{"name":"domain","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,2,], errors: [1,,], leaf: 7 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();

export const prerendered = new Set([]);
