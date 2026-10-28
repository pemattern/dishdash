import { api } from "./request";

export type GetUserResponse = {
  id: string;
  email: string;
  createdAt: string;
  updatedAt: string;
};

export const users = {
  async me() {
    return await api.get<GetUserResponse>('/users/me');
  }
}
