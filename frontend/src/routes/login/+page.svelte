<script lang="ts">
  import { Button } from 'positron-components/components/ui/button';
  import * as Card from 'positron-components/components/ui/card';
  import { toast } from 'positron-components/components/util/general';

  let { data } = $props();

  $effect(() => {
    const url = new URL(window.location.href);
    let updated = false;
    if (data.error) {
      let error = '';
      switch (data.error) {
        case 'missing_code':
          error = 'SSO login failed: Missing authorization code.';
          break;
        default:
          error = `SSO login failed: ${data.error}`;
      }

      toast.error(error);

      url.searchParams.delete('error');
      updated = true;
    }
    if (data.skip) {
      url.searchParams.delete('skip');
      updated = true;
    }
    if (updated) {
      window.history.replaceState({}, '', url);
    }
  });
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
  <Card.Root class="mx-auto w-full max-w-sm">
    <Card.Header>
      <Card.Title class="text-2xl">Login</Card.Title>
      <Card.Description
        >Click the button below to login with OIDC.</Card.Description
      >
    </Card.Header>
    <Card.Content>
      <Button
        variant="outline"
        class="w-full cursor-pointer"
        onclick={() => {
          if (!data.oidc_url) {
            toast.error('Failed to get OIDC URL.');
            return;
          }
          window.location.href = data.oidc_url;
        }}>Login with OIDC</Button
      >
    </Card.Content>
  </Card.Root>
</div>
