<script lang="ts">
	import { X } from 'lucide-svelte';

	interface Props {
		open: boolean;
		title: string;
		onClose: () => void;
		size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl';
		children?: any;
	}

	let { open = $bindable(false), title, onClose, size = 'md', children }: Props = $props();

	const sizeClasses = {
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl',
		'2xl': 'max-w-6xl'
	};

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && open) {
			onClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4"
		onclick={handleBackdropClick}
		role="presentation"
	>
		<div class="bg-slate-800 rounded-lg shadow-2xl w-full {sizeClasses[size]} max-h-[90vh] overflow-hidden flex flex-col border border-slate-700">
			<!-- Header -->
			<div class="flex items-center justify-between p-6 border-b border-slate-700">
				<h2 class="text-xl font-semibold text-slate-100">{title}</h2>
				<button
					onclick={onClose}
					class="text-slate-400 hover:text-slate-100 transition-colors p-1 hover:bg-slate-700 rounded"
					aria-label="Close modal"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Content -->
			<div class="flex-1 overflow-y-auto p-6">
				{@render children()}
			</div>
		</div>
	</div>
{/if}
