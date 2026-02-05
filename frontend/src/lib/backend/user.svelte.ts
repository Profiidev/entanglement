import { get, ResponseType } from 'positron-components/backend';

export interface UserInfo {
  uuid: string;
  name: string;
  email: string;
  avatar?: string;
}

export const getUserInfo = async (
  fetch: typeof window.fetch = window.fetch
) => {
  return await get<UserInfo>('/api/user', {
    res_type: ResponseType.Json,
    fetch
  });
};
