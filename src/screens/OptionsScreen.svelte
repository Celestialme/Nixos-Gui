

    <div class='container' class:populated={filteredKey.length}>
        <input type="text" bind:value={$OptionInputValue} >
        
        {#each filteredKey as  key }
        <Option name={key} description={$optionList[key].description} example={$optionList[key].example} defaultValue={$optionList[key].default} type={$optionList[key].type }/>
        {/each}







    </div>








<style>
   input{
        display:block;
        width:50%;
        margin:0 auto;
        font-size: 30px;
        border-radius: 6px;
        border:1px solid;
        outline: none;
        text-align: center;
        margin-bottom: 20px;
        margin-top: -30px;
    }
    .populated{
    margin: 10px;
    
    border: 2px solid;
    border-radius: 12px;
    padding: 10px;
    }
    .container{
        margin-top:10px;
        min-height: 91%;
        padding: 10px;
    }
</style>







<script lang='ts'>
import { onMount } from "svelte";
import axios from 'axios';
import Option from "@src/components/Option.svelte";
import { optionList ,OptionInputValue} from "@src/store/store";
let filteredKey:Array<any>=[];
let worker:Worker;
let keys:Array<any>=[];
onMount(async ()=>{
 $optionList  = await axios.get('options.json').then(data=>data.data)
keys= Object.keys($optionList);
worker = new Worker('worker.js');
worker.onmessage = function({data}){
if($OptionInputValue=='')return
if(data.type!='filterOptions')return
filteredKey = data.value;
}
filter($OptionInputValue)
})

$:filter($OptionInputValue)
   
function filter($OptionInputValue){
     
    if(!worker || $OptionInputValue==''){
        filteredKey=[]
        return
    }
    
    worker.postMessage({type:'filterOptions',payload:{keys,$optionList,value:$OptionInputValue}})
}




</script>