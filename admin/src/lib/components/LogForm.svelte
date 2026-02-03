<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	// @ts-ignore - ToastUI Editor doesn't have TypeScript definitions
	import Editor from '@toast-ui/editor';
	import '@toast-ui/editor/dist/toastui-editor.css';
	import type { Log, LogCreateInput, LogUpdateInput } from '$lib/types';

	interface Props {
		initialData?: Log | null;
		onSubmit: (data: LogCreateInput) => Promise<void> | void;
		onCancel: () => void;
		isLoading?: boolean;
	}

	let { initialData = null, onSubmit, onCancel, isLoading = false }: Props = $props();

	let editorElement: HTMLDivElement;
	let editor: Editor | null = null;
	let errors = $state<Record<string, string>>({});
	let characterCount = $state(0);

	onMount(() => {
		if (editorElement) {
			editor = new Editor({
				el: editorElement,
				height: '600px',
				initialEditType: 'wysiwyg',
				previewStyle: 'vertical',
				initialValue: initialData?.content || '',
				placeholder: 'Enter log content...',
				hideModeSwitch: false,
				toolbarItems: [
					['heading', 'bold', 'italic', 'strike'],
					['hr', 'quote'],
					['ul', 'ol', 'task'],
					['table', 'link', 'image'],
					['code', 'codeblock'],
					['scrollSync']
				],
				theme: 'dark',
				events: {
					change: () => {
						characterCount = editor?.getMarkdown()?.length || 0;
					}
				}
			});

			// Set initial character count
			characterCount = editor?.getMarkdown()?.length || 0;
		}
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});

	function validate(): boolean {
		const newErrors: Record<string, string> = {};
		const content = editor?.getMarkdown() || '';

		if (!content.trim()) {
			newErrors.content = 'Content is required';
		} else if (content.trim().length < 10) {
			newErrors.content = 'Content must be at least 10 characters';
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	function handleSubmit(e: Event) {
		e.preventDefault();

		if (!validate()) {
			return;
		}

		const content = editor?.getMarkdown() || '';
		const data: LogCreateInput = {
			content: content.trim()
		};

		onSubmit(data);
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<!-- ToastUI Editor -->
	<div>
		<label for="editor" class="block text-sm font-medium text-slate-300 mb-2">
			Content <span class="text-red-400">*</span>
		</label>
		<div
			id="editor"
			bind:this={editorElement}
			class="rounded-lg overflow-hidden border border-slate-700 {isLoading
				? 'opacity-50 pointer-events-none'
				: ''}"
		></div>
		<p class="mt-2 text-xs text-slate-500">
			Supports Markdown formatting • {characterCount} characters
		</p>
		{#if errors.content}
			<p class="mt-1 text-sm text-red-400">{errors.content}</p>
		{/if}
	</div>

	<!-- Actions -->
	<div class="flex gap-3 pt-4">
		<button
			type="submit"
			disabled={isLoading}
			class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{isLoading ? 'Saving...' : initialData ? 'Update Log' : 'Create Log'}
		</button>
		<button
			type="button"
			onclick={onCancel}
			disabled={isLoading}
			class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			Cancel
		</button>
	</div>
</form>

<style>
	/* Dark theme customization for ToastUI Editor */
	:global(.toastui-editor-defaultUI) {
		background-color: rgb(15 23 42); /* slate-900 */
		border: 1px solid rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-toolbar) {
		background-color: rgb(30 41 59); /* slate-800 */
		border-bottom: 1px solid rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-toolbar-icons) {
		color: rgb(203 213 225); /* slate-300 */
	}

	:global(.toastui-editor-toolbar-icons:hover) {
		background-color: rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-main-container) {
		background-color: rgb(15 23 42); /* slate-900 */
	}

	:global(.toastui-editor-md-container),
	:global(.toastui-editor-ww-container) {
		background-color: rgb(15 23 42); /* slate-900 */
	}

	:global(.toastui-editor-md-container .toastui-editor .ProseMirror),
	:global(.toastui-editor-ww-container .toastui-editor .ProseMirror) {
		color: rgb(226 232 240); /* slate-200 */
		background-color: rgb(15 23 42); /* slate-900 */
	}

	:global(.toastui-editor-md-preview) {
		background-color: rgb(15 23 42); /* slate-900 */
		color: rgb(226 232 240); /* slate-200 */
	}

	:global(.toastui-editor-md-tab-container) {
		background-color: rgb(30 41 59); /* slate-800 */
	}

	:global(.toastui-editor-md-tab-container .tab-item) {
		color: rgb(148 163 184); /* slate-400 */
	}

	:global(.toastui-editor-md-tab-container .tab-item.active) {
		color: rgb(226 232 240); /* slate-200 */
		background-color: rgb(15 23 42); /* slate-900 */
	}

	:global(.toastui-editor-md-vertical-style .toastui-editor-md-splitter) {
		background-color: rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-popup) {
		background-color: rgb(30 41 59); /* slate-800 */
		border: 1px solid rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-popup-body) {
		color: rgb(226 232 240); /* slate-200 */
	}

	/* Code blocks */
	:global(.toastui-editor-md-preview pre),
	:global(.toastui-editor-contents pre) {
		background-color: rgb(30 41 59); /* slate-800 */
		border: 1px solid rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-md-preview code),
	:global(.toastui-editor-contents code) {
		background-color: rgb(30 41 59); /* slate-800 */
		color: rgb(248 113 113); /* red-400 */
	}

	/* Links */
	:global(.toastui-editor-md-preview a),
	:global(.toastui-editor-contents a) {
		color: rgb(96 165 250); /* blue-400 */
	}

	/* Headings */
	:global(.toastui-editor-md-preview h1),
	:global(.toastui-editor-md-preview h2),
	:global(.toastui-editor-md-preview h3),
	:global(.toastui-editor-contents h1),
	:global(.toastui-editor-contents h2),
	:global(.toastui-editor-contents h3) {
		color: rgb(226 232 240); /* slate-200 */
		border-bottom-color: rgb(51 65 85); /* slate-700 */
	}

	/* Tables */
	:global(.toastui-editor-md-preview table),
	:global(.toastui-editor-contents table) {
		border-color: rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-md-preview th),
	:global(.toastui-editor-md-preview td),
	:global(.toastui-editor-contents th),
	:global(.toastui-editor-contents td) {
		border-color: rgb(51 65 85); /* slate-700 */
	}

	:global(.toastui-editor-md-preview th),
	:global(.toastui-editor-contents th) {
		background-color: rgb(30 41 59); /* slate-800 */
	}

	/* Blockquotes */
	:global(.toastui-editor-md-preview blockquote),
	:global(.toastui-editor-contents blockquote) {
		border-left-color: rgb(100 116 139); /* slate-500 */
		color: rgb(148 163 184); /* slate-400 */
	}

	/* Scrollbar */
	:global(.toastui-editor-md-container::-webkit-scrollbar),
	:global(.toastui-editor-ww-container::-webkit-scrollbar),
	:global(.toastui-editor-md-preview::-webkit-scrollbar) {
		width: 8px;
		height: 8px;
	}

	:global(.toastui-editor-md-container::-webkit-scrollbar-track),
	:global(.toastui-editor-ww-container::-webkit-scrollbar-track),
	:global(.toastui-editor-md-preview::-webkit-scrollbar-track) {
		background: rgb(15 23 42); /* slate-900 */
	}

	:global(.toastui-editor-md-container::-webkit-scrollbar-thumb),
	:global(.toastui-editor-ww-container::-webkit-scrollbar-thumb),
	:global(.toastui-editor-md-preview::-webkit-scrollbar-thumb) {
		background: rgb(51 65 85); /* slate-700 */
		border-radius: 4px;
	}

	:global(.toastui-editor-md-container::-webkit-scrollbar-thumb:hover),
	:global(.toastui-editor-ww-container::-webkit-scrollbar-thumb:hover),
	:global(.toastui-editor-md-preview::-webkit-scrollbar-thumb:hover) {
		background: rgb(71 85 105); /* slate-600 */
	}
</style>
