import { get, post, ResponseType } from 'positron-components/backend';

export const testToken = async () => {
  let res = await get<boolean>('/api/auth/test_token', {
    res_type: ResponseType.Json
  });

  if (typeof res === 'boolean') {
    return res;
  }
};

export const getOidcUrl = async (redirect: string) => {
  let res = await post<{ url: string }>('/api/auth/oidc/url', {
    res_type: ResponseType.Json,
    body: { redirect }
  });

  if (typeof res === 'object') {
    return res.url;
  }
};
