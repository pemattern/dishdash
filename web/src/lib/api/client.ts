import { appState } from '#lib/state.svelte.ts';
import type { ErrorSchema } from '$bindings';
import { ApiError } from './error';

const apiPath = '/api';

const ACCEPT_HEADER_KEY = 'Accept';
const CONTENT_TYPE_HEADER_KEY = 'Content-Type';
const AUTHORIZATION_HEADER_KEY = 'Authorization';

const APPLICATION_JSON_HEADER_VALUE = 'application/json';

export class ApiException extends Error {}

function id_path(path: string, id: string): string {
	return `${path}/${id}`;
}

export const GetAll = <Schema>(path: string) => ({
	get_all: (): Promise<Schema[]> => api.get<Schema[]>(path)
});

export const GetById = <Schema>(path: string) => ({
	get_by_id: (id: string): Promise<Schema> => api.get<Schema>(id_path(path, id))
});

export const Create = <Schema, Post>(path: string) => ({
	create: (schema: Post): Promise<Schema> => api.post<Schema>(path, schema)
});

export const BulkCreate = <Schema, Post>(path: string) => ({
	bulk_create: (schemas: Record<string, Post>): Promise<Schema[]> =>
		api.post<Schema[]>(path, schemas)
});

export const Update = <Schema, Patch>(path: string) => ({
	update: (id: string, schema: Patch): Promise<Schema> =>
		api.patch<Schema>(id_path(path, id), schema)
});

export const BulkUpdate = <Schema, Patch>(path: string) => ({
	bulk_update: (schemas: Record<string, Patch>): Promise<Schema[]> =>
		api.patch<Schema[]>(path, schemas)
});

export const Delete = (path: string) => ({
	delete: (id: string): Promise<null> => api.delete(id_path(path, id))
});

export const api = {
	get: <T>(path: string) =>
		request<T>(path, {
			headers: getHeaders(false)
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

	delete: (path: string) =>
		request<null>(path, {
			method: 'DELETE',
			headers: getHeaders(false)
		})
};

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
	let response: Response;
	try {
		response = await fetch(`${apiPath}${path}`, options);
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

function getHeaders(withContent: boolean): HeadersInit {
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
	return `Bearer ${appState.accessToken}`;
}
