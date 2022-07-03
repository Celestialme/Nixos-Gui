<script lang="ts">
import { ast, changes, needsSaving } from "@src/store/store";

import {  Ast2Text, find_key_value,setContainerHeight } from "@src/utils/globalFunctions";
import { invoke } from "@tauri-apps/api/tauri";
export let name;
let value=[];
let _value = $changes[name] || find_key_value($ast,name,"node").findNode?.("self","NODE_LIST")?.children.filter(node => !node.kind.startsWith("TOKEN")).map(node=>Ast2Text(node))
console.log(_value)
// if(_value){ 
//     let _JSON =  _value.replace(/\s*⇐change|\s*⇐ADD/g,'').trim().replace(/\"/g,'\\"').replace(/\s+/g,'","').replace(/^(\[\s*)",/,'$1').replace(/,"\s*(\])$/,'$1')
//     value= JSON.parse(_JSON).map((item)=>({'value':item}))
    
// }

if(_value && !_value.js){ 
    
    value= _value.map((item)=>({'value':item}))
    
}else if(_value){
    value = _value.js
}else{
    invoke("repl_command",{payload: 'builtins.toJSON config.'+name }).then((data:string)=>{ListEntry=JSON.parse(JSON.parse(data)).map((item)=>({'value':item,discard:true}))})
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
   
    
    let listToString = "[ "+ ListEntry.reduce((acc,x)=>{if (x.discard==true) return acc; return acc+' '+x.value+' '},'')+" ]"
    console.log(ListEntry)
    $changes[name]={nix:listToString,js:ListEntry}
    $needsSaving=true;
}

</script>
<div class='container' use:setContainerHeight>

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
    width:90vw;
    padding-top: 8px;
    overflow-y: auto;
    overflow-x: hidden;
    background: #F8F8F8;
    border-radius: 13px;
    margin:auto;
    border-top-left-radius: 0;
    border-top-right-radius: 0;
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