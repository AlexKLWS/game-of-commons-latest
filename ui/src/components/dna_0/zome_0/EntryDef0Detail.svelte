<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { InstalledCell, EntryHash, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { appInfoContext, appWebsocketContext } from '../../../contexts';
import { EntryDef0 } from '../../../types/dna_0/zome_0';
import '@type-craft/title/title-detail';
import '@type-craft/content/content-detail';

export let entryHash: EntryHash;

let appInfo = getContext(appInfoContext).getAppInfo();
let appWebsocket = getContext(appWebsocketContext).getAppWebsocket();

let entryDef0: EntryDef0 | undefined;

$: entryDef0;

onMount(async () => {
  const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'dna_0')!;

  entryDef0 = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: cellData.cell_id,
    zome_name: 'zome_0',
    fn_name: 'get_entry_def_0',
    payload: entryHash,
    provenance: cellData.cell_id[1]
  });
});
</script>

{#if entryDef0}
  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">EntryDef0</span>

    
    <title-detail
    
      value={entryDef0.title}
      style="margin-top: 16px"
    ></title-detail>

    
    <content-detail
    
      value={entryDef0.content}
      style="margin-top: 16px"
    ></content-detail>

  </div>
{:else}
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{/if}
