

    <div class='container' class:populated={filteredKey.length}>
        
        
        {#each filteredKey as  key }
        <Option name={key} description={$optionList[key.replace(/<.*>/,'<name>')]?.description} example={$optionList[key.replace(/<.*>/,'<name>')]?.example} defaultValue={$optionList[key.replace(/<.*>/,'<name>')]?.default} type={$optionList[key.replace(/<.*>/,'<name>')]?.type }/>
        {/each}







    </div>








<style>
  
   
   
</style>







<script lang='ts'>
import { onMount } from "svelte";
import axios from 'axios';
import Option from "@src/components/Option.svelte";
import { optionList ,OptionInputValue} from "@src/store/store";
import { invoke } from "@tauri-apps/api/tauri";

let filteredKey:Array<any>=[];
let worker:Worker;
let keys:Array<any>=[];
export let inputValue;
$:$OptionInputValue = inputValue;
onMount(async ()=>{
 $optionList  = JSON.parse(await invoke('get_options'))
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