import i18next, {
	type i18n as I18N,
	type InitOptions,
	type LanguageDetectorModule,
	type Services
} from 'i18next';
import resourcesToBackend from 'i18next-resources-to-backend';
import { browser } from '$app/environment';

const LOCALE_FILE_REGEX = /^\/src\/locales\/(\w+)\/([a-zA-Z]{2}([_-][a-zA-Z]{2})?)\.json$/;

function process_locales(): {
	translations: Record<string, () => Promise<any>>;
	locales: string[];
	namespaces: string[];
} {
	const files = import.meta.glob('/src/locales/*/*.json', {
		exhaustive: false,
		eager: false,
		import: 'default'
	});
	const languages = new Set<string>();
	const namespaces = new Set<string>();
	const translations_entries = Object.entries(files).map(([path, loader]) => {
		const [, namespace, locale] = LOCALE_FILE_REGEX.exec(path) || [];
		if (!namespace || !locale) throw new Error(`Invalid locale file path: ${path}`);
		languages.add(locale);
		namespaces.add(namespace);
		return [`${namespace}:${locale}`, loader];
	});
	const translations = Object.fromEntries(translations_entries);
	return {
		translations,
		locales: Array.from(languages),
		namespaces: Array.from(namespaces)
	};
}

export const { translations, locales, namespaces } = process_locales();

async function load_translation(locale: string, namespace: string): Promise<any | undefined> {
	const key = `${namespace}:${locale}`;
	if (browser) console.time(`[i18n] Load translation ${key}`);
	try {
		const translation = translations[key];
		if (translation) {
			return await translation();
		} else {
			return undefined;
		}
	} finally {
		if (browser) console.timeEnd(`[i18n] Load translation ${key}`);
	}
}

export const CONTEXT_CURRENT_LOCALE = 'current-locale';
export const CONTEXT_I18N = 'i18n';

const backend = resourcesToBackend(load_translation);

function createDetector(params: I18nCreateParams): LanguageDetectorModule {
	return {
		type: 'languageDetector',
		init(services: Services, detectorOptions: object, i18nextOptions: InitOptions) {
			if (browser)
				console.debug('[i18n] Detector init', { services, detectorOptions, i18nextOptions });
		},
		detect(): string | readonly string[] | undefined {
			return params.browser_lang;
		}
	};
}

type I18nCreateParams = {
	readonly browser_lang: string[] | string | undefined;
};

export async function create_i18n(params: I18nCreateParams): Promise<I18N> {
	const detector = createDetector(params);
	try {
		if (browser) {
			console.time('[i18n] Create instance');
			console.debug('[i18n] Create i18n', { params });
		}
		const i18n = i18next.createInstance().use(detector).use(backend);
		await i18n.init({
			debug: false,
			ns: namespaces,
			initImmediate: false,
			supportedLngs: locales,
			defaultNS: 'common',
			fallbackNS: ['common', 'meta'],
			fallbackLng: 'en',
			nsSeparator: ':',
			load: 'all',
			cleanCode: true,
			lowerCaseLng: true,
			nonExplicitSupportedLngs: true,
			maxParallelReads: 10
		});
		await load_all_meta_translations(i18n);
		return i18n;
	} finally {
		if (browser) console.timeEnd('[i18n] Create instance');
	}
}

async function load_all_meta_translations(i18n: I18N) {
	for (const locale of locales) {
		if (!i18n.getResourceBundle(locale, 'meta')) {
			//console.debug(`[i18n] missing meta for ${locale}`);
			await i18n.reloadResources(locale, 'meta');
		}
	}
}
