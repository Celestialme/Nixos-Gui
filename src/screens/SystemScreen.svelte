<div class="buttons-container">
    <button on:click={()=>state.value=state.channels}>channels</button>
    <button on:click={()=>state.value=state.generations}>generations</button>
    <button on:click={()=>invoke("update_channels")}>update channels</button>
    <button on:click={()=>invoke("rebuild_switch")} >rebuild</button>
</div>

{#if state.value == state.channels}
<Channels bind:state={state}/>

{:else if state.value == state.generations}
<Generations bind:state={state}/>
{/if}


<script lang='ts'>
import Channels from "@src/components/Channels.svelte";
import Generations from "@src/components/Generations.svelte";
import { invoke } from '@tauri-apps/api/tauri'

let state={
    value:"none",
    channels: "channels",
    generations: "generations",
    none:"none"
}
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