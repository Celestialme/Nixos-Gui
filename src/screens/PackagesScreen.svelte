

    <div class='container' class:populated={filteredKey.length}>
        <button class:showInstalled={showInstalled} on:click={()=>{showInstalled=!showInstalled;filter()}}>SHOW INSTALLED</button>
        <!-- <input type="text" bind:value={inputValue} on:keyup={filter}> -->
        
        {#each filteredKey as  key }
        <Package name={getKeyName(key,$overhead)} description={packages[key].description} version={packages[key].version}/>
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
import { installedPkgs, overhead } from "@src/store/store";
export let inputValue:String='';
export const keyUpFn:Function=filter;
let packages=[];
let filteredKey:Array<any>=[];
let worker:Worker;
let keys:Array<any>=[];
let showInstalled:boolean = false;

onMount(async ()=>{
    // $nixEnvPkgs=await invoke("get_nix_env_packages")
 packages  = await axios.get('packagesList.json').then(data=>data.data)
keys= Object.keys(packages);
$overhead = getOverhead(keys)
worker = new Worker('/worker.js');
worker.onmessage = function({data}){
if(inputValue=='')return
filteredKey = data.value;
}
})

$:filteredKey =filterInstalledPackages(showInstalled)
   
function filter(){
    console.log(inputValue)
    if(!worker || (inputValue=='' && !showInstalled)){
        filteredKey=[]
        return
    }
    if(!showInstalled){
    worker.postMessage({type:'filterPackages',payload:{keys,packages,value:inputValue}})
    }else{
        let _filteredKey = filterInstalledPackages(showInstalled)
        if(inputValue==''){
            filteredKey=_filteredKey
            }
        worker.postMessage({type:'filterPackages',payload:{keys:_filteredKey,packages,value:inputValue}})

    }
}


function filterInstalledPackages(showInstalled){
let result = []
if(!showInstalled || !keys.length)return[]
let prefix = keys[0].split('.').slice(0,$overhead-1).join('.')+'.'
for (let i = 0; i < $installedPkgs.length; i++) {
        let pkg = prefix+$installedPkgs[i]
        
        if(!packages[pkg]) continue
        result.push(pkg)
}
console.log(result)

return result
}

</script>