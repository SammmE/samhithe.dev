// Auth Store using Svelte 5 Runes
import { browser } from '$app/environment';

const STORAGE_KEY = 'admin_password';

class AuthStore {
	password = $state<string | null>(null);

	constructor() {
		// Load password from localStorage on initialization
		if (browser) {
			const stored = localStorage.getItem(STORAGE_KEY);
			if (stored) {
				this.password = stored;
			}
		}
	}

	login(password: string) {
		this.password = password;
		if (browser) {
			localStorage.setItem(STORAGE_KEY, password);
		}
	}

	logout() {
		this.password = null;
		if (browser) {
			localStorage.removeItem(STORAGE_KEY);
		}
	}

	isAuthenticated(): boolean {
		return this.password !== null && this.password.length > 0;
	}

	getPassword(): string | null {
		return this.password;
	}
}

// Export a singleton instance
export const authStore = new AuthStore();
