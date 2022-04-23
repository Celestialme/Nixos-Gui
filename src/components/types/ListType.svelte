<script>
import { ast, changes, needsSaving } from "@src/store/store";

import {  find_key_value } from "@src/utils/globalFunctions";
export let name;
let value=[];
let _value = $changes[name] || find_key_value($ast,name)[1];
console.log($ast);
if(_value){ 
    let _JSON =  _value.replace(/\s*⇐change|\s*⇐ADD/g,'').trim().replace(/\s+/g,',').replace(/^(\[\s*),/,'$1').replace(/,\s*(\])$/,'$1')
    value= JSON.parse(_JSON).map((item)=>({'value':item}))
    
}
let inputValue=''
let ListEntry=value.length?value:[];


function add(){
    if(!inputValue) return
    ListEntry.push({value:inputValue})
    ListEntry=ListEntry
    
    inputValue=''
    change()
}
function remove(entry){
    ListEntry = ListEntry.filter((x)=>x!=entry)
    change()
}
function change(){
   
    
    let listToString = "[ "+ ListEntry.reduce((acc,x)=>acc+' "'+x.value+'" ','')+" ]"
    $changes[name]=listToString
    $needsSaving=true;
}

</script>
<div class='container'>

    <div>
        <input type="text" placeholder={'ADD Entry'} bind:value={inputValue}>
        <button on:click={add}>ADD</button>
    </div>

    <div class='list-container'>
        {#each ListEntry  as  entry}
        <div>
            <input type="text" value={entry.value}>
            <button on:click={()=>remove(entry)}>Remove</button>
        </div>
        {/each}



    </div>









</div>

<style>
.container{
    text-align: center;
}

input{
    width: 30%;
    text-align: center;
    border-radius: 8px;
    border: 1px solid;
    padding: 10px;
    margin: 10px 0;
    font-size: 20px;
    border-bottom-right-radius:0 ;
    border-top-right-radius: 0
}
button{
    min-width: 100px;
    font-size: 20px;
    padding: 10px;
    border-radius: 8px;
    border:1px solid;
    border-bottom-left-radius:0 ;
    border-top-left-radius: 0;
    margin-left: -3px;
    
}
button:active{
    transform: scale(0.9);
}
</style>