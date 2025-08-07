<script lang="ts">
	import { onMount } from 'svelte';
	import { marked } from 'marked';
	import { markedHighlight } from 'marked-highlight';
	import hljs from 'highlight.js/lib/core';
	
	// Import specific languages for better performance
	import bash from 'highlight.js/lib/languages/bash';
	import javascript from 'highlight.js/lib/languages/javascript';
	import typescript from 'highlight.js/lib/languages/typescript';
	import python from 'highlight.js/lib/languages/python';
	import rust from 'highlight.js/lib/languages/rust';
	import json from 'highlight.js/lib/languages/json';
	import yaml from 'highlight.js/lib/languages/yaml';
	import xml from 'highlight.js/lib/languages/xml';
	
	interface Props {
		content: string;
	}
	
	let { content }: Props = $props();
	let renderedContent = $state('');
	
	onMount(() => {
		// Register languages
		hljs.registerLanguage('bash', bash);
		hljs.registerLanguage('shell', bash);
		hljs.registerLanguage('sh', bash);
		hljs.registerLanguage('javascript', javascript);
		hljs.registerLanguage('js', javascript);
		hljs.registerLanguage('typescript', typescript);
		hljs.registerLanguage('ts', typescript);
		hljs.registerLanguage('python', python);
		hljs.registerLanguage('py', python);
		hljs.registerLanguage('rust', rust);
		hljs.registerLanguage('rs', rust);
		hljs.registerLanguage('json', json);
		hljs.registerLanguage('yaml', yaml);
		hljs.registerLanguage('yml', yaml);
		hljs.registerLanguage('xml', xml);
		hljs.registerLanguage('html', xml);
		
		// Configure marked with syntax highlighting
		marked.use(markedHighlight({
			langPrefix: 'hljs language-',
			highlight(code, lang) {
				const language = hljs.getLanguage(lang) ? lang : 'plaintext';
				return hljs.highlight(code, { language }).value;
			}
		}));
		
		// Configure marked options
		marked.setOptions({
			breaks: true,
			gfm: true
		});
		
		renderContent();
	});
	
	function renderContent() {
		try {
			renderedContent = marked(content);
		} catch (error) {
			console.error('Error rendering markdown:', error);
			renderedContent = content; // Fallback to plain text
		}
	}
	
	// Re-render when content changes
	$effect(() => {
		if (content) {
			renderContent();
		}
	});
</script>

<svelte:head>
	<!-- Highlight.js theme -->
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/styles/github.min.css" media="(prefers-color-scheme: light)">
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/styles/github-dark.min.css" media="(prefers-color-scheme: dark)">
</svelte:head>

<div class="prose prose-sm max-w-none dark:prose-invert">
	{@html renderedContent}
</div>

<style>
	:global(.prose) {
		color: inherit;
	}
	
	:global(.prose h1, .prose h2, .prose h3, .prose h4, .prose h5, .prose h6) {
		color: inherit;
		margin-top: 1.5em;
		margin-bottom: 0.5em;
	}
	
	:global(.prose h1:first-child, .prose h2:first-child, .prose h3:first-child) {
		margin-top: 0;
	}
	
	:global(.prose p) {
		margin-top: 0.75em;
		margin-bottom: 0.75em;
	}
	
	:global(.prose p:first-child) {
		margin-top: 0;
	}
	
	:global(.prose p:last-child) {
		margin-bottom: 0;
	}
	
	:global(.prose pre) {
		background-color: rgb(243 244 246);
		border: 1px solid rgb(229 231 235);
		border-radius: 0.5rem;
		padding: 1rem;
		overflow-x: auto;
		margin: 1em 0;
	}
	
	:global(.dark .prose pre) {
		background-color: rgb(31 41 55);
		border-color: rgb(75 85 99);
	}
	
	:global(.prose code) {
		background-color: rgb(243 244 246);
		padding: 0.125rem 0.25rem;
		border-radius: 0.25rem;
		font-size: 0.875em;
	}
	
	:global(.dark .prose code) {
		background-color: rgb(31 41 55);
	}
	
	:global(.prose pre code) {
		background-color: transparent;
		padding: 0;
		border-radius: 0;
		font-size: inherit;
	}
	
	:global(.prose ul, .prose ol) {
		margin: 1em 0;
		padding-left: 1.5em;
	}
	
	:global(.prose li) {
		margin: 0.25em 0;
	}
	
	:global(.prose blockquote) {
		border-left: 4px solid rgb(229 231 235);
		padding-left: 1rem;
		margin: 1em 0;
		font-style: italic;
		color: rgb(107 114 128);
	}
	
	:global(.dark .prose blockquote) {
		border-left-color: rgb(75 85 99);
		color: rgb(156 163 175);
	}
	
	:global(.prose table) {
		width: 100%;
		border-collapse: collapse;
		margin: 1em 0;
	}
	
	:global(.prose th, .prose td) {
		border: 1px solid rgb(229 231 235);
		padding: 0.5rem;
		text-align: left;
	}
	
	:global(.dark .prose th, .dark .prose td) {
		border-color: rgb(75 85 99);
	}
	
	:global(.prose th) {
		background-color: rgb(249 250 251);
		font-weight: 600;
	}
	
	:global(.dark .prose th) {
		background-color: rgb(31 41 55);
	}
</style>
