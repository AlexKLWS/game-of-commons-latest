<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import { AppWebsocket, EntryHash, InstalledAppInfo } from '@holochain/client';
  import '@material/mwc-circular-progress';

  import { appWebsocketContext, appInfoContext } from './contexts';
  import CreateEntryDef0 from './components/dna_0/zome_0/CreateEntryDef0.svelte';
import EntryDef0Detail from './components/dna_0/zome_0/EntryDef0Detail.svelte';

  let appWebsocket: AppWebsocket | undefined;
  let appInfo: InstalledAppInfo | undefined;
  let loading = true;
  let entryHash: EntryHash | undefined;

  $: appWebsocket, appInfo, entryHash, loading;

  onMount(async () => {
    appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

    appInfo = await appWebsocket.appInfo({
      installed_app_id: 'game-of-commons-devcamp',
    });
    loading = false;
  });

  setContext(appWebsocketContext, {
    getAppWebsocket: () => appWebsocket,
  });

  setContext(appInfoContext, {
    getAppInfo: () => appInfo,
  });
</script>

<main>
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <CreateEntryDef0 on:entry-def-0-created="{e => entryHash = e.detail.entryHash}"></CreateEntryDef0>

    {#if entryHash}
      <EntryDef0Detail entryHash={entryHash}></EntryDef0Detail>
    {/if}
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
