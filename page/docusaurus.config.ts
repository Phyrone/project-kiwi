import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

//latex support
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';

const config: Config = {
	title: 'Project Kiwi',
	tagline: 'stay tuned',
	favicon: 'img/favicon.ico',

	// Set the production url of your site here
	url: 'https://project-kiwi.phyrone.eu/',
	// Set the /<baseUrl>/ pathname under which your site is served
	// For GitHub pages deployment, it is often '/<projectName>/'
	baseUrl: '/',

	// GitHub pages deployment config.
	// If you aren't using GitHub pages, you don't need these.
	//organizationName: 'Phyrone', // Usually your GitHub org/user name.
	//projectName: 'project-kiwi', // Usually your repo name.

	onBrokenLinks: 'throw',
	onBrokenMarkdownLinks: 'warn',
	trailingSlash: true,

	// Even if you don't use internationalization, you can use this field to set
	// useful metadata like html lang. For example, if your site is Chinese, you
	// may want to replace "en" with "zh-Hans".
	i18n: {
		defaultLocale: 'en',
		locales: ['en']
	},
	markdown: {
		mermaid: true
	},

	presets: [
		[
			'classic',
			{
				docs: {
					sidebarPath: './sidebars.ts',
					breadcrumbs: true,
					editUrl:
						'https://github.com/Phyrone/project-kiwi/tree/main/page/',
					remarkPlugins: [remarkMath],
					rehypePlugins: [rehypeKatex]
				},
				blog: {
					showReadingTime: true,
					feedOptions: {
						type: 'all'
					},
					editUrl:
						'https://github.com/Phyrone/project-kiwi/tree/main/page/',
					remarkPlugins: [remarkMath],
					rehypePlugins: [rehypeKatex]
				},
				theme: {
					customCss: './src/css/custom.css'
				}
			} satisfies Preset.Options
		]
	],
	plugins: [
		[
			'@docusaurus/plugin-ideal-image',
			{}
		]
	],
	themes: ['@docusaurus/theme-mermaid'],
	themeConfig: {
		// Replace with your project's social card
		image: 'img/docusaurus-social-card.jpg',
		navbar: {
			title: 'Project Kiwi',
			/*logo: {
				alt: 'My Site Logo',
				src: 'img/logo.svg',
			},*/
			items: [
				{
					type: 'docSidebar',
					sidebarId: 'tutorialSidebar',
					position: 'left',
					label: 'Docs'
				},
				{
					to: '/blog',
					label: 'Blog',
					position: 'left'
				},
				{
					type: 'localeDropdown',
					position: 'right'
				},
				{
					href: 'https://github.com/phyrone/project-kiwi',
					label: 'GitHub',
					position: 'right'
				}
			]
		},
		footer: {
			style: 'dark',
			links: [
				/*
				{
					title: 'Community',
					items: [
						{
							label: 'Stack Overflow',
							href: 'https://stackoverflow.com/questions/tagged/docusaurus'
						},
						{
							label: 'Discord',
							href: 'https://discordapp.com/invite/docusaurus'
						},
						{
							label: 'Twitter',
							href: 'https://twitter.com/docusaurus'
						}
					]
				}, */
				{
					title: 'Legal Stuff',
					items: [
						{
							label: 'Impressum',
							to: '/impressum/'
						},
						{
							label: 'GitHub',
							href: 'https://github.com/facebook/docusaurus'
						}
					]
				}
			]
			//copyright: `Copyright © ${new Date().getFullYear()} Phyrone. Built with Docusaurus.`
		},
		prism: {
			theme: prismThemes.github,
			darkTheme: prismThemes.dracula
		}
	} satisfies Preset.ThemeConfig,
	stylesheets: [
		{
			href: '/assets/katex/katex.min.css',
			type: 'text/css'
		}
	]
};

export default config;
