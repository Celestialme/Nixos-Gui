

    <div class='container'>
        
        
        {#each filtered_options as  option }
        <Option name={option.key} description={option?.description} example={option?.example} defaultValue={option?.default} type={option?.type }/>
        {/each}







    </div>








<style>
  
   
   
</style>







<script lang='ts'>
import { onMount ,onDestroy } from "svelte";
import Option from "@src/components/Option.svelte";
import { OptionInputValue} from "@src/store/store";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

let filtered_options:Array<any>=[];
export let inputValue;
$:$OptionInputValue = inputValue;
let unlisten;
onMount(async ()=>{


listen('filterOptions', (e:any) => {
if($OptionInputValue=='')return
filtered_options =  e.payload.map(x=>JSON.parse(x));
console.log(filtered_options)
}).then(_unlisten=>unlisten=_unlisten)

filter($OptionInputValue)
})

onDestroy(()=>{
    unlisten()
    
})

$:filter($OptionInputValue)
   
function filter($OptionInputValue){
     
    if( $OptionInputValue==''){
        filtered_options=[]
        return
    }
    
    // worker.postMessage({type:'filterOptions',payload:{keys,$optionList,value:$OptionInputValue}})
     invoke("filter_options",{value:$OptionInputValue})
}







</script>