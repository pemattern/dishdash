import { auth } from "#lib";

const apiPath = '/api';

type RequestOptions = Omit<RequestInit, 'method' | 'body'>;

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
	const response = await fetch(`${apiPath}${path}`, {
		...options,
    headers: {
      'Authorization': `Bearer ${auth.accessToken}`,
			'Content-Type': 'application/json',
			...options.headers
		}
	});
	if (!response.ok) {
		throw new Error(`API error: ${response.status}`);
	}
	if (response.status === 204) return undefined as T;
	return response.json() as Promise<T>;
}

export const api = {
	get: <T>(path: string, options: RequestOptions = {}) => request<T>(path, options),

	post: <T>(path: string, body: unknown, options: RequestOptions = {}) =>
		request<T>(path, {
			method: 'POST',
			body: JSON.stringify(body),
			...options
		}),

	patch: <T>(path: string, body: unknown, options: RequestOptions = {}) =>
		request<T>(path, {
			method: 'PATCH',
			body: JSON.stringify(body),
			...options
		}),

	delete: <T>(path: string, options: RequestOptions = {}) =>
		request<T>(path, { method: 'DELETE', ...options })
};
