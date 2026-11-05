import type { ErrorSchema } from '$bindings';

export class ApiError extends Error {
	constructor(public readonly response: ErrorSchema) {
		super(response.detail ?? response.title);
	}

	get status(): number {
		return this.response.status;
	}
}
