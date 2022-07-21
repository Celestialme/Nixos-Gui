

    <div class='container' >
        <button class:showInstalled={showInstalled} on:click={()=>{showInstalled=!showInstalled;filter()}}>SHOW INSTALLED</button>
        
        <!-- <input type="text" bind:value={inputValue} on:keyup={filter}> -->
        
        {#each [{key:"nixos.google-chrome",description:"Google chrome browser",version:"1.0.0"}] as  pkg }
         
        <Package name={pkg.key} description={pkg.description} version={pkg.version}/>
        
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
import { onMount ,onDestroy } from "svelte";
import axios from 'axios';

import { markedPkgs, nixEnvPkgs,currentScreen, needs_db_update } from "@src/store/store";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
export let inputValue:String='';
export const keyUpFn:Function=filter;
let packages={error:null};
let filtered_pkgs:Array<any>=[];
let worker:Worker;
let keys:Array<any>=[];
let showInstalled:boolean = false;

let unlisten;
onMount(async ()=>{
    $nixEnvPkgs=await invoke("get_nix_env_pkgs")
 packages  = JSON.parse( await invoke('get_packages')) 
 if(packages.error){
     $currentScreen=2
     $needs_db_update=true;
 }
keys= Object.keys(packages);
worker = new Worker('/worker.js');
// worker.onmessage = function({data}){
// if(inputValue=='')return
// filteredKey = data.value;
// }

listen('filterPackages', (e:any) => {
    if(inputValue=='' && !showInstalled)return
filtered_pkgs = e.payload.map(x=>JSON.parse(x))
}).then(_unlisten=>unlisten=_unlisten)

})
onDestroy(()=>{
    
   unlisten && unlisten()
    
})


$:{filter()
showInstalled
}
   
function filter(){
    
    if( inputValue=='' && !showInstalled){
        filtered_pkgs=[]
        return
    }
    if(!showInstalled){
        console.log("filtering1")
    invoke("filter_packages",{value:inputValue,keys:[]})
    }else{
        
       console.log("filtering2")
        // worker.postMessage({type:'filterPackages',payload:{keys:_filteredKey,packages,value:inputValue}})
       
        invoke("filter_packages",{value:inputValue,keys:[...$nixEnvPkgs,...$markedPkgs]})

    }
}





</script>
