<script lang="ts">
	import { page } from '$app/stores';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { LayoutDashboard, FolderGit2, FileText, LogOut } from 'lucide-svelte';

	const navItems = [
		{ href: '/', label: 'Dashboard', icon: LayoutDashboard },
		{ href: '/projects', label: 'Projects', icon: FolderGit2 },
		{ href: '/logs', label: 'Logs', icon: FileText }
	];

	function handleLogout() {
		authStore.logout();
		goto('/login');
	}
</script>

<aside class="fixed left-0 top-0 h-screen w-64 bg-slate-800 border-r border-slate-700 flex flex-col">
	<!-- Logo/Title -->
	<div class="p-6 border-b border-slate-700">
		<h1 class="text-xl font-bold text-slate-100">Portfolio Admin</h1>
		<p class="text-sm text-slate-400 mt-1">Dashboard</p>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 p-4 space-y-1">
		{#each navItems as item}
			{@const Icon = item.icon}
			<a
				href={item.href}
				class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {$page.url
					.pathname === item.href
					? 'bg-blue-600 text-white'
					: 'text-slate-300 hover:bg-slate-700 hover:text-slate-100'}"
			>
				<Icon size={20} />
				<span class="font-medium">{item.label}</span>
			</a>
		{/each}
	</nav>

	<!-- Logout Button -->
	<div class="p-4 border-t border-slate-700">
		<button
			onclick={handleLogout}
			class="flex items-center gap-3 px-4 py-3 w-full rounded-lg text-slate-300 hover:bg-red-600/10 hover:text-red-400 transition-colors"
		>
			<LogOut size={20} />
			<span class="font-medium">Logout</span>
		</button>
	</div>
</aside>
