

    <div class='container' >
        <button class:showInstalled={showInstalled} on:click={()=>{showInstalled=!showInstalled;filter(showInstalled)}}>SHOW INSTALLED</button>
        <button class:showInstalled={showInstalled} on:click={runInvoke}>run invoke</button>
        <!-- <input type="text" bind:value={inputValue} on:keyup={filter}> -->
        
        {#each filtered_pkgs as  pkg }
         
        <Package name={getKeyName(pkg.key,$overhead)} description={pkg.description} version={pkg.version}/>
        
        {/each}





        

    </div>








<style>
 
    button{
    display: block;
    margin: auto;
    padding: 10px;
    font-size: 20px;
    border-radius: 7px;
    border: 1px solid;
    color: white;
    background: #2e272eb8
    }
    .showInstalled{
        background-color: green;
    }
    button:active{
        transform: scale(0.9)
    }
</style>







<script lang='ts'>
import Package from "@src/components/Package.svelte";
import { onMount } from "svelte";
import axios from 'axios';
import {getKeyName, getOverhead} from '@src/utils/globalFunctions'
import { installedPkgs, nixEnvPkgs, overhead,currentScreen, needs_db_update } from "@src/store/store";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
export let inputValue:String='';
export const keyUpFn:Function=filter;
let packages={error:null};
let filtered_pkgs:Array<any>=[];
let worker:Worker;
let keys:Array<any>=[];
let showInstalled:boolean = false;

onMount(async ()=>{
    $nixEnvPkgs=await invoke("get_nix_env_pkgs")
 packages  = JSON.parse( await invoke('get_packages')) 
 if(packages.error){
     $currentScreen=2
     $needs_db_update=true;
 }
keys= Object.keys(packages);
$overhead = getOverhead(keys)
worker = new Worker('/worker.js');
// worker.onmessage = function({data}){
// if(inputValue=='')return
// filteredKey = data.value;
// }

let unlisten;
listen('filterPackages', (e:any) => {
    if(inputValue=='')return
filtered_pkgs = e.payload.map(x=>JSON.parse(x))
}).then(_unlisten=>unlisten=_unlisten)

})

$:filter(showInstalled)
   
function filter(showInstalled){
    
    if(!worker || (inputValue=='' && !showInstalled)){
        filtered_pkgs=[]
        return
    }
    if(!showInstalled){
   
    invoke("filter_packages",{value:inputValue,keys:[]})
    }else{
        
        console.log($nixEnvPkgs)
        // worker.postMessage({type:'filterPackages',payload:{keys:_filteredKey,packages,value:inputValue}})
        invoke("filter_packages",{value:inputValue,keys:$nixEnvPkgs})

    }
}




function runInvoke(){
    invoke("exp",{pkgs:["1","2","3"]})
}

</script>
