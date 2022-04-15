

    <div class='container' class:populated={filteredKey.length}>
        <input type="text" bind:value={inputValue} on:keyup={filter}>
        
        {#each filteredKey as  key }
        <Package name={packages[key].pname} description={packages[key].description} version={packages[key].version}/>
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
    margin-top: 60px !important;
    border: 2px solid #00000073;
    border-radius: 12px;
    
    }
    .container{
    margin-top: 20px;
    min-height: 91%;
    padding: 30px 50px;
    transition: 0.5s;
    margin-top: 60px;
    }
</style>







<script lang='ts'>
import Package from "@src/components/Package.svelte";
import { onMount } from "svelte";
import axios from 'axios';
let packages=[];
let filteredKey:Array<any>=[];
let inputValue:String='';
let worker:Worker;
let keys:Array<any>=[];
onMount(async ()=>{
 packages  = await axios.get('packagesList.json').then(data=>data.data)
keys= Object.keys(packages);
worker = new Worker('/worker.js');
worker.onmessage = function({data}){
if(inputValue=='')return
filteredKey = data.value;
}
})


   
function filter(){

    if(!worker || inputValue==''){
        filteredKey=[]
        return
    }
 worker.postMessage({type:'filterPackages',payload:{keys,packages,value:inputValue}})
}




</script>