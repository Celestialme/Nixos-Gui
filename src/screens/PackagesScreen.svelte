

    <div class='container' class:populated={filteredKey.length}>
        <button class:showInstalled={showInstalled} on:click={()=>{showInstalled=!showInstalled;filter()}}>SHOW INSTALLED</button>
        <button class:showInstalled={showInstalled} on:click={runInvoke}>run invoke</button>
        <!-- <input type="text" bind:value={inputValue} on:keyup={filter}> -->
        
        {#each filteredKey as  key }
         {#if packages[key]}
        <Package name={getKeyName(key,$overhead)} description={packages[key].description} version={packages[key].version}/>
         {/if}
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
let filteredKey:Array<any>=[];
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
worker.onmessage = function({data}){
if(inputValue=='')return
filteredKey = data.value;
}

let unlisten;
listen('filterPackages', (e:any) => {
let pkgs = e.payload.map(x=>JSON.parse(x))
console.log(pkgs)
}).then(_unlisten=>unlisten=_unlisten)

})

$:filteredKey =filterInstalledPackages(showInstalled)
   
function filter(){
    console.log(inputValue)
    if(!worker || (inputValue=='' && !showInstalled)){
        filteredKey=[]
        return
    }
    if(!showInstalled){
   
    invoke("filter_packages",{value:inputValue,keys:[]})
    }else{
        let _filteredKey = filterInstalledPackages(showInstalled)
        if(inputValue==''){
            filteredKey=_filteredKey
            }
        worker.postMessage({type:'filterPackages',payload:{keys:_filteredKey,packages,value:inputValue}})

    }
}


function filterInstalledPackages(showInstalled){
    console.log($nixEnvPkgs)
let result = [...$nixEnvPkgs.map((pkg)=>keys[0] && keys[0].split('.').slice(0,$overhead-1).join(".")+"."+pkg)]

if(!showInstalled || !keys.length)return[]
let prefix = keys[0].split('.').slice(0,$overhead-1).join('.')+'.'
for (let i = 0; i < $installedPkgs.length; i++) {
        let pkg = prefix+$installedPkgs[i]
        
        if(!packages[pkg]) continue
        result.push(pkg)
}
console.log(result)

return Array.from(new Set(result)) 
}


function runInvoke(){
    invoke("exp",{pkgs:["1","2","3"]})
}

</script>
