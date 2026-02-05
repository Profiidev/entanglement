import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types.js';
import { noSidebarPaths } from '$lib/components/navigation/sidebar/items.svelte';

export const load: LayoutServerLoad = ({ cookies, url }) => {
  let cookie = cookies.get('entanglement_jwt');

  if (!cookie && !noSidebarPaths.includes(url.pathname)) {
    redirect(302, '/login');
  }
};
