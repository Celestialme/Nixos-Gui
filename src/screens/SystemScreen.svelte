<div class="buttons-container">
    <button on:click={()=>state.value=state.channels}>channels</button>
    <button on:click={()=>state.value=state.generations}>generations</button>
    <button class:error={update_channels_success==false} on:click={update_channels}>update channels</button>
    <button on:click={()=>rebuild_switch(false)} >rebuild</button>
    {#if showProgress}
    <ProgressBar  value={value} {msg} max_value={max_value} {success}/>
    {/if}
</div>

{#if state.value == state.channels}
<Channels bind:state={state}/>

{:else if state.value == state.generations}
<Generations bind:state={state}/>
{/if}


<script lang='ts'>
import Channels from "@src/components/Channels.svelte";
import Generations from "@src/components/Generations.svelte";
import ProgressBar from "@src/components/ProgressBar.svelte";
import { listen } from "@tauri-apps/api/event";
import { invoke } from '@tauri-apps/api/tauri'
import { onDestroy } from "svelte";

let state={
    value:"none",
    channels: "channels",
    generations: "generations",
    none:"none"
}
let update_channels_success;
async function update_channels(){
  let data =  await invoke("update_channels").then((data:string)=>JSON.parse(data))
  update_channels_success = data.success
}

let showProgress=false
    let showPromt = false;
    let value = 0
    let max_value =1
    let success;
    let msg = ''
    let unlisten;
function rebuild_switch(password) {
    showPromt=true;
    if(!password) return
    console.log(password)
    if(showProgress)return
    showProgress=true;
    
listen('progress-rebuild-switch', (e:any) => {
  console.log(e.payload)
  let data = JSON.parse(e.payload)
  value = data.progress[0]
  max_value = data.progress[1]
  msg = data.msg
}).then(_unlisten=>unlisten=_unlisten)
listen('finish-rebuild-switch', async (e:any) => {
    success=e.payload; 
}).then(_unlisten=>unlisten=_unlisten)
invoke("rebuild_switch",{password:password})
}
onDestroy(()=>{
  unlisten && unlisten()
})
</script>

<style>
    .buttons-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 30px;
      
    }
    .error{
        background:red;
    }
    .error:hover{
        background: orange;
    }
    button{
        height:100px;
        width:300px;
        background:#403F3F;
        color:white;
        border-radius: 6px;
        font-size: 25px;
    }
    button:hover{
        background:#403f3fc4
    }
    button:active{
        transform: scale(0.9);
    }

</style>