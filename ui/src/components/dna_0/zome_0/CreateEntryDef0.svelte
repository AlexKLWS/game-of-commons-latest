<script lang="ts">
import { createEventDispatcher, getContext } from 'svelte';
import '@material/mwc-button';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';

import { appWebsocketContext, appInfoContext } from '../../../contexts';
import { EntryDef0 } from '../../../types/dna_0/zome_0';
import '@type-craft/title/create-title';
import '@type-craft/content/create-content';

let appInfo = getContext(appInfoContext).getAppInfo();
let appWebsocket = getContext(appWebsocketContext).getAppWebsocket();

const dispatch = createEventDispatcher();

let title: string | undefined;
let content: string | undefined;

$: title, content;

async function createEntryDef0() {
  const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'dna_0')!;

  const entryDef0: EntryDef0 = {
    title: title!,
        content: content!,
  };

  
  const { entryHash } = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: cellData.cell_id,
    zome_name: 'zome_0',
    fn_name: 'create_entry_def_0',
    payload: entryDef0,
    provenance: cellData.cell_id[1]
  });

  dispatch('entry-def-0-created', { entryHash });
}

</script>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create EntryDef0</span>

  <create-title
      
      on:change="{e => title = e.target.value}"
      style="margin-top: 16px"
    ></create-title>

  <create-content
      
      on:change="{e => content = e.target.value}"
      style="margin-top: 16px"
    ></create-content>

  <mwc-button 
    label="Create EntryDef0"
    disabled={!(title && content)}
    on:click="{() => createEntryDef0()}"
  ></mwc-button>
</div>
