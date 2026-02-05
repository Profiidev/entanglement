import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getOidcUrl } from '$lib/backend/auth.svelte';

export const load: PageLoad = async ({ url }) => {
  let error = url.searchParams.get('error') || null;
  if (error) {
    return { error };
  }
  let skip = url.searchParams.get('skip') === 'true';

  let oidcUrl = await getOidcUrl(url.searchParams.get('redirect') || '/');
  if (oidcUrl && !skip) {
    redirect(302, oidcUrl);
  }
  return { oidc_url: oidcUrl, skip };
};
