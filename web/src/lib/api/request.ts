import { auth } from './auth.svelte';

const apiPath = '/api';

const ACCEPT_HEADER_KEY = 'Accept';
const CONTENT_TYPE_HEADER_KEY = 'Content-Type';
const AUTHORIZATION_HEADER_KEY = 'Authorization';

const APPLICATION_JSON_HEADER_VALUE = 'application/json';

function getHeaders(withContent = false): HeadersInit {
	let headers: HeadersInit = {
		[ACCEPT_HEADER_KEY]: APPLICATION_JSON_HEADER_VALUE,
		[AUTHORIZATION_HEADER_KEY]: getBearerTokenHeaderValue()
	};
	if (withContent) {
		headers = {
			[CONTENT_TYPE_HEADER_KEY]: APPLICATION_JSON_HEADER_VALUE,
			...headers
		};
	}
	return headers;
}

function getBearerTokenHeaderValue(): string {
	return `Bearer ${auth.accessToken}`;
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
	const response = await fetch(`${apiPath}${path}`, options);

	if (options.headers)
		if (!response.ok) {
			throw new Error(`API error: ${response.status}`);
		}
	if (response.status === 204) return undefined as T;
	return response.json() as Promise<T>;
}

export const api = {
	get: <T>(path: string) =>
		request<T>(path, {
			headers: getHeaders()
		}),

	post: <T>(path: string, body: unknown) =>
		request<T>(path, {
			method: 'POST',
			body: JSON.stringify(body),
			headers: getHeaders(true)
		}),

	patch: <T>(path: string, body: unknown) =>
		request<T>(path, {
			method: 'PATCH',
			body: JSON.stringify(body),
			headers: getHeaders(true)
		}),

	delete: <T>(path: string) =>
		request<T>(path, {
			method: 'DELETE',
			headers: getHeaders()
		})
};
