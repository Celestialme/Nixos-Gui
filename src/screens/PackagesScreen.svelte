

    <div class='container' class:populated={filteredKey.length}>
        <button class:showInstalled={showInstalled} on:click={()=>{showInstalled=!showInstalled;filter()}}>SHOW INSTALLED</button>
        <input type="text" bind:value={inputValue} on:keyup={filter}>
        
        {#each filteredKey as  key }
        <Package name={getKeyName(key,$overhead)} description={packages[key].description} version={packages[key].version}/>
        {/each}





        

    </div>








<style>
   input{
        display: block;
        width: 50%;
        margin: 0 auto;
        font-size: 30px;
        border-radius: 6px;
        border: 1px solid;
        outline: none;
        text-align: center;
        margin-bottom: 20px;
        margin-top: -60px;
        padding: 10px;
    }
    .populated{
     margin: 10px;
    margin-top: 70px !important;
    border: 2px solid #00000073;
    border-radius: 12px;
    
    }
    .container{
    margin-top: 20px;
    min-height: 91%;
    padding: 30px 50px;
    transition: 0.5s;
    margin-top: 73px;
    }
    button{
    display: block;
    margin: auto;
    margin-top: -110px;
    margin-bottom: 65px;
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
let packages=[];
let filteredKey:Array<any>=[];
let inputValue:String='';
let worker:Worker;
let keys:Array<any>=[];
let showInstalled:boolean = false;
onMount(async ()=>{
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