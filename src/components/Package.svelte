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
        <ProgressBar title="building..." value=50 max_value=450/>
        <ProgressBar title="copying..." value={value} max_value=450/>
        <ProgressBar title="Downloading..." value=50 max_value=450/>
        {/if}
    </div>

</div>


<script>
import { invoke } from "@tauri-apps/api/tauri";

import CheckIcon from "./icons/CheckIcon.svelte";
import DownloadIcon from "./icons/DownloadIcon.svelte";
import ProgressBar from "./ProgressBar.svelte";



    export let name;
    export let description;
    export let version ;
    let showProgress=false;
    let value = 100


function startDownload() {
    showProgress=true;
    invoke('start_download',{payload:name})
}
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