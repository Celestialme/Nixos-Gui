<div class="container">
    <div class="icon-panel">
        <img src={img_src} on:error={try_url} alt="">
        
    </div>
    <div class='content-panel'>
        <p class='name'>{@html getKeyName(name.replace(/\./g,".<wbr>"))}</p>
        <p class='description'>{description}</p>
        <p class='version'>version: {version}</p>
        {#key name}
        {#if showProgress}
        <ProgressBar  value={value} {msg} max_value={max_value} {success}/>
        {/if}
        {/key}
    </div>
    <div class='right-panel'>
        {#key name}
            <CheckIcon {name}/>
            <DownloadIcon  {marked} on:click={()=>startDownload(name)}/>
        {/key}
    </div>
</div>


<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import CheckIcon from "./icons/CheckIcon.svelte";
import DownloadIcon from "./icons/DownloadIcon.svelte";
import ProgressBar from "./ProgressBar.svelte";
import { onDestroy } from "svelte";
import { nixEnvPkgs} from "@src/store/store";
import { getKeyName } from "@src/utils/globalFunctions";



    export let name;
    export let description;
    export let version ;
    let showProgress=false;
    let value = 0
    let max_value =1
    let success;
    let msg = ''
    let unlisten;
    let marked
$:{showProgress=false;
     value = 0
     max_value =1
     success=undefined;
     msg = '';
     marked=$nixEnvPkgs.includes(name);
    name //dependency
}

function startDownload(name) {
    if(showProgress)return
    name = getKeyName(name);
    showProgress=true;
    
listen('progress-'+name.replace(/\./g,''), (e:any) => {
  console.log(e.payload)
  let data = JSON.parse(e.payload)
  value = data.progress[0]
  max_value = data.progress[1]
  msg = data.msg
}).then(_unlisten=>unlisten=_unlisten)
listen('finish-'+name.replace(/\./g,''), async (e:any) => {
    success=e.payload;
    if(success!="true")return
   $nixEnvPkgs=await invoke("get_nix_env_pkgs")
 
}).then(_unlisten=>unlisten=_unlisten)

invoke('start_download',{payload:name})
}
onDestroy(()=>{
  unlisten && unlisten()
})

let try_count=0
let img_src ="static/pkg_icons/"+getKeyName(name)+".png"
function try_url(){
    try_count++;
    switch (try_count){
        case 1: 
         img_src ="static/pkg_icons/"+getKeyName(name)+".svg"
        break
        case 2: 
         img_src ="static/pkg_icons/"+getKeyName(name)+".gif"
        break
    }
}

</script>

<style>
    .right-panel{
        display:flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;
        width:60px;
        background: rgba(0, 0, 0, 0.07);
    }
    .content-panel{
        display:flex;
        padding: 20px 30px;
        min-height:150px;
        flex-direction: column;
        justify-content: space-around;
        flex-grow: 1;
    }
    .icon-panel{
        width:100px
    }
    .icon-panel img, picture{
        width:100px;
       margin-top:20px;
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
    width: 800px;
       
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