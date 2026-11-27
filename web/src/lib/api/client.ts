import { ensureAuthenticated } from './auth';
import { fetch } from './transport';

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

export async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
	await ensureAuthenticated();
	return await fetch(path, options);
}
