<div class="container">
    <div class='left-panel'>
        {#key name}
            <CheckIcon {name}/>
        {/key}
            <DownloadIcon on:click={()=>startDownload()}/>
    </div>
    <div class='right-panel'>
        <p class='name'>{@html name.replace(/\./g,".<wbr>")}</p>
        <p class='description'>{description}</p>
        <p class='version'>version: {version}</p>
        {#if showProgress}
        <ProgressBar title="building..." value={value} max_value={max_value} {success}/>
        {/if}
    </div>

</div>


<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import CheckIcon from "./icons/CheckIcon.svelte";
import DownloadIcon from "./icons/DownloadIcon.svelte";
import ProgressBar from "./ProgressBar.svelte";
import { onDestroy } from "svelte";



    export let name;
    export let description;
    export let version ;
    let showProgress=false;
    let value = 0
    let max_value =1
    let success;
let unlisten;
function startDownload() {
    if(showProgress)return
    showProgress=true;
listen('progress-'+name.replace(/\./g,''), (e:any) => {
  [value,max_value] = JSON.parse(e.payload)

 
}).then(_unlisten=>unlisten=_unlisten)
listen('finish-'+name.replace(/\./g,''), (e:any) => {
    success=e.payload;
    console.log(success)
 
}).then(_unlisten=>unlisten=_unlisten)

invoke('start_download',{payload:name})
}
onDestroy(()=>{
  unlisten && unlisten()
})

</script>

<style>
    .left-panel{
        display:flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;
        width:60px;
        background: rgba(0, 0, 0, 0.07);
    }
    .right-panel{
        display:flex;
        padding: 20px 30px;
        min-height:150px;
        flex-direction: column;
        justify-content: space-around;
        flex-grow: 1;
    }
    .container{
    display: flex;
    position: relative;
    text-align: center;
    margin: 10px auto;
    border-bottom: 1px solid;
    border-radius: 12px;
    box-shadow: 3px 2px 10px 1px #00000066;
    cursor: default;
    width: 720px;
       
    }
    .name{
        font-size: 37px;
        margin-bottom: 20px;
        
    }
    .description,.version{
        font-size: 20px;
    }
    .version{
        margin-top: auto;
    }
    p{
    
        text-align: center;
    }
   
</style>