<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import { toast } from 'svelte-sonner';
	import { Lock } from 'lucide-svelte';

	let password = $state('');
	let isLoading = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!password.trim()) {
			toast.error('Please enter a password');
			return;
		}

		isLoading = true;

		try {
			// Save password and attempt to authenticate
			authStore.login(password);

			// Test the password by making a simple API call
			const response = await fetch('http://localhost:3000/stats', {
				headers: {
					'X-Admin-Password': password
				}
			});

			if (!response.ok) {
				throw new Error('Authentication failed');
			}

			toast.success('Login successful!');
			goto('/');
		} catch (error) {
			authStore.logout();
			toast.error('Invalid password');
			password = '';
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="min-h-screen bg-slate-900 flex items-center justify-center p-4">
	<div class="w-full max-w-md">
		<!-- Logo/Title -->
		<div class="text-center mb-8">
			<div class="inline-flex items-center justify-center w-16 h-16 bg-blue-600 rounded-full mb-4">
				<Lock size={32} class="text-white" />
			</div>
			<h1 class="text-3xl font-bold text-slate-100 mb-2">Portfolio Admin</h1>
			<p class="text-slate-400">Enter your admin password to continue</p>
		</div>

		<!-- Login Card -->
		<div class="bg-slate-800 rounded-lg shadow-2xl p-8 border border-slate-700">
			<form onsubmit={handleSubmit} class="space-y-6">
				<div>
					<label for="password" class="block text-sm font-medium text-slate-300 mb-2">
						Admin Password
					</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						placeholder="Enter your password"
						disabled={isLoading}
						autocomplete="current-password"
						class="w-full px-4 py-3 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
					/>
				</div>

				<button
					type="submit"
					disabled={isLoading}
					class="w-full px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
				>
					{#if isLoading}
						<div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
						<span>Logging in...</span>
					{:else}
						<Lock size={18} />
						<span>Login</span>
					{/if}
				</button>
			</form>
		</div>

		<!-- Footer -->
		<p class="text-center text-slate-500 text-sm mt-6">
			Protected by header-based authentication
		</p>
	</div>
</div>
