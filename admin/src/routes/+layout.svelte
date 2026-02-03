<script lang="ts">
	import './layout.css';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { Toaster } from 'svelte-sonner';

	let { children } = $props();

	// Check authentication on route changes
	$effect(() => {
		const currentPath = $page.url.pathname as string;
		const isLoginPage = currentPath === '/login';
		const isAuthenticated = authStore.isAuthenticated();

		// Redirect to login if not authenticated (unless already on login page)
		if (!isAuthenticated && !isLoginPage) {
			goto('/login');
		}

		// Redirect to home if authenticated and on login page
		if (isAuthenticated && isLoginPage) {
			goto('/');
		}
	});
</script>

<Toaster
	position="top-right"
	theme="dark"
	richColors
	closeButton
/>

{#if ($page.url.pathname as string) === '/login'}
	<!-- Login page without sidebar -->
	{@render children()}
{:else}
	<!-- Dashboard layout with sidebar -->
	<div class="flex h-screen bg-slate-900 overflow-hidden">
		<Sidebar />
		<main class="flex-1 ml-64 overflow-y-auto">
			<div class="p-8">
				{@render children()}
			</div>
		</main>
	</div>
{/if}
