<script lang="ts">
	import { marked } from 'marked';

	interface Props {
		content: string;
	}

	let { content }: Props = $props();

	// Configure marked for better rendering
	marked.setOptions({
		breaks: true,
		gfm: true
	});

	let renderedHtml = $derived(marked(content || 'No README content available.'));
</script>

<div class="prose prose-invert prose-slate max-w-none
	[&]:text-slate-300
	[&_h1]:text-slate-100 [&_h1]:border-b [&_h1]:border-slate-700 [&_h1]:pb-2
	[&_h2]:text-slate-100 [&_h2]:border-b [&_h2]:border-slate-700 [&_h2]:pb-2 [&_h2]:mt-8
	[&_h3]:text-slate-200 [&_h3]:mt-6
	[&_a]:text-blue-400 [&_a]:hover:text-blue-300 [&_a]:no-underline
	[&_code]:bg-slate-900 [&_code]:text-emerald-400 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:rounded [&_code]:text-sm
	[&_pre]:bg-slate-900 [&_pre]:border [&_pre]:border-slate-700 [&_pre]:rounded-lg
	[&_pre_code]:bg-transparent [&_pre_code]:text-slate-300 [&_pre_code]:p-0
	[&_blockquote]:border-l-4 [&_blockquote]:border-blue-500 [&_blockquote]:bg-slate-800/50 [&_blockquote]:italic
	[&_ul]:list-disc [&_ul]:pl-6
	[&_ol]:list-decimal [&_ol]:pl-6
	[&_table]:border-collapse [&_table]:w-full
	[&_th]:bg-slate-800 [&_th]:border [&_th]:border-slate-700 [&_th]:px-4 [&_th]:py-2 [&_th]:text-left
	[&_td]:border [&_td]:border-slate-700 [&_td]:px-4 [&_td]:py-2
	[&_img]:rounded-lg [&_img]:border [&_img]:border-slate-700
">
	{@html renderedHtml}
</div>
