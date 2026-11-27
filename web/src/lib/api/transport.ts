import { appState } from '#lib/state.svelte.ts';
import type { ErrorSchema } from '$bindings';
import { ApiError } from './error';

const apiPath = '/api';

const ACCEPT_HEADER_KEY = 'Accept';
const CONTENT_TYPE_HEADER_KEY = 'Content-Type';
const AUTHORIZATION_HEADER_KEY = 'Authorization';

const APPLICATION_JSON_HEADER_VALUE = 'application/json';

let globalFetch: typeof fetch;

export function setFetch(fetch: typeof globalThis.fetch) {
	globalFetch = fetch;
}

export function getFetch() {
	return globalFetch;
}

export async function fetch<T>(path: string, options: RequestInit = {}): Promise<T> {
	let response: Response;
	try {
		const headers = new Headers(options.headers);
		headers.set(ACCEPT_HEADER_KEY, APPLICATION_JSON_HEADER_VALUE);
		if (!headers.has(AUTHORIZATION_HEADER_KEY)) {
			headers.set(AUTHORIZATION_HEADER_KEY, getBearerTokenHeaderValue());
		}
		if (options.body) {
			headers.set(CONTENT_TYPE_HEADER_KEY, APPLICATION_JSON_HEADER_VALUE);
		}
		response = await globalFetch(`${apiPath}${path}`, {
			...options,
			headers
		});
	} catch (network_error) {
		// TODO: handle network error
		console.error(network_error);
		throw network_error;
	}
	if (!response.ok) {
		const errorSchema = (await response.json()) as ErrorSchema;
		throw new ApiError(errorSchema);
	}
	if (response.status === 204) return undefined as T;
	return response.json() as Promise<T>;
}

function getBearerTokenHeaderValue(): string {
	return `Bearer ${appState.accessToken}`;
}
