import { get, ResponseType } from 'positron-components/backend';

export const testToken = async () => {
  let res = await get<boolean>('/api/auth/test_token', {
    res_type: ResponseType.Json
  });

  if (typeof res === 'boolean') {
    return res;
  }
};

export const getOidcUrl = async () => {
  let res = await get<{ url: string }>('/api/auth/oidc/url', {
    res_type: ResponseType.Json
  });

  if (typeof res === 'object') {
    return res.url;
  }
};
