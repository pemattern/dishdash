import { appState } from '#lib/state.svelte.ts';
import type { ErrorSchema } from '$bindings';
import { ensureAuthenticated } from './auth';
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

function id_path(path: string, id: string): string {
	return `${path}/${id}`;
}

export const GetAll = <Schema>(path: string) => ({
	getAll: (): Promise<Schema[]> => methods.get<Schema[]>(path)
});

export const GetById = <Schema>(path: string) => ({
	getById: (id: string): Promise<Schema> => methods.get<Schema>(id_path(path, id))
});

export const Create = <Schema, Post>(path: string) => ({
	create: (schema: Post): Promise<Schema> => methods.post<Schema>(path, schema)
});

export const BulkCreate = <Schema, Post>(path: string) => ({
	bulkCreate: (schemas: Record<string, Post>): Promise<Schema[]> =>
		methods.post<Schema[]>(path, schemas)
});

export const Update = <Schema, Patch>(path: string) => ({
	update: (id: string, schema: Patch): Promise<Schema> =>
		methods.patch<Schema>(id_path(path, id), schema)
});

export const BulkUpdate = <Schema, Patch>(path: string) => ({
	bulkUpdate: (schemas: Record<string, Patch>): Promise<Schema[]> =>
		methods.patch<Schema[]>(path, schemas)
});

export const Delete = (path: string) => ({
	delete: (id: string): Promise<null> => methods.delete(id_path(path, id))
});

export const methods = {
	get: <T>(path: string) => request<T>(path),

	post: <T>(path: string, body: unknown) =>
		request<T>(path, {
			method: 'POST',
			body: JSON.stringify(body)
		}),

	patch: <T>(path: string, body: unknown) =>
		request<T>(path, {
			method: 'PATCH',
			body: JSON.stringify(body)
		}),

	delete: (path: string) =>
		request<null>(path, {
			method: 'DELETE'
		})
};

export async function request<T>(
	path: string,
	options: RequestInit = {},
	skipAuthentication = false
): Promise<T> {
	if (!skipAuthentication) {
		await ensureAuthenticated();
	}
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
