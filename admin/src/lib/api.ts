// API Client using Axios with auto-injected auth header
import axios, { type AxiosError } from 'axios';
import { authStore } from './stores/auth.svelte';
import { goto } from '$app/navigation';
import type {
	Stats,
	Project,
	Log,
	ProjectCreateInput,
	ProjectUpdateInput,
	LogCreateInput,
	LogUpdateInput
} from './types';

// Create axios instance
export const apiClient = axios.create({
	baseURL: 'http://localhost:3000',
	headers: {
		'Content-Type': 'application/json'
	}
});

// Request interceptor to inject auth header
apiClient.interceptors.request.use(
	(config) => {
		const password = authStore.getPassword();
		if (password) {
			config.headers['X-Admin-Password'] = password;
		}
		return config;
	},
	(error) => {
		return Promise.reject(error);
	}
);

// Response interceptor for global error handling
apiClient.interceptors.response.use(
	(response) => response,
	(error: AxiosError) => {
		// Handle 401 Unauthorized
		if (error.response?.status === 401) {
			authStore.logout();
			goto('/login');
		}
		return Promise.reject(error);
	}
);

// API Functions

// Stats
export async function getStats(): Promise<Stats> {
	const response = await apiClient.get<Stats>('/stats');
	return response.data;
}

// Projects
export async function getProjects(): Promise<Project[]> {
	const response = await apiClient.get<Project[]>('/projects');
	return response.data;
}

export async function createProject(data: ProjectCreateInput): Promise<Project> {
	const response = await apiClient.post<Project>('/admin/projects', data);
	return response.data;
}

export async function updateProject(id: number, data: ProjectUpdateInput): Promise<Project> {
	const response = await apiClient.put<Project>(`/admin/projects/${id}`, data);
	return response.data;
}

export async function deleteProject(id: number): Promise<void> {
	await apiClient.delete(`/admin/projects/${id}`);
}

export async function refreshProjectReadme(id: number): Promise<void> {
	await apiClient.post(`/admin/projects/${id}/refresh-readme`);
}

// Logs
export async function getLogs(page: number = 1, limit: number = 20): Promise<Log[]> {
	const response = await apiClient.get<Log[]>('/logs', {
		params: { page, limit }
	});
	return response.data;
}

export async function createLog(data: LogCreateInput): Promise<Log> {
	const response = await apiClient.post<Log>('/admin/logs', data);
	return response.data;
}

export async function updateLog(id: number, data: LogUpdateInput): Promise<Log> {
	const response = await apiClient.put<Log>(`/admin/logs/${id}`, data);
	return response.data;
}

export async function deleteLog(id: number): Promise<void> {
	await apiClient.delete(`/admin/logs/${id}`);
}

// Admin Operations
export async function forceSync(): Promise<void> {
	await apiClient.post('/admin/sync');
}
