<script>
import { ast, changes, needsSaving } from "@src/store/store";

import {  find_key_value, removeLastChar ,setContainerHeight} from "@src/utils/globalFunctions";
export let name;
let value=[];
let _value =  $changes[name]?.nix || find_key_value($ast,name)[1];


if(_value){ 
    let _JSON =  removeLastChar(',',_value.replace(/\s*⇐change|\s*⇐ADD/g,'').replace(/\"/g,'\\"').replace(/=/g,':').replace(/;/g,',').replace(/(.*:)/g,'"$1"').replace(/(:.*),/g,'"$1",'))
    value= Object.entries(JSON.parse(_JSON)).map((item)=>({key:item[0],value:item[1]}))
    
}

let ListEntry=value.length?value:[];
let inputValue=''
let textAreaValue=''
function add(){
    if(!inputValue || !textAreaValue) return
    ListEntry.push({key:inputValue,value:textAreaValue})
    ListEntry=ListEntry
    
    inputValue=''
    textAreaValue=''
    change()
}
function remove(entry){
    ListEntry = ListEntry.filter((x)=>x!=entry)
    change()
}
function change(){
  let attrToString = '{\n'+ ListEntry.reduce((acc,x)=>{
    return acc + x.key+'  = '+x.value+';\n'
  },'') + '\n}'
$changes[name]={nix:attrToString}
$needsSaving=true;
}

</script>

<div class='container' style="text-align: center;" use:setContainerHeight>   
    <div class='attr-container'>
        <div class='row'>
            <p>Key</p>
            <button on:click={add}>ADD</button>
           <input type="text" bind:value={inputValue}> 
           
        </div>
        <span> : </span>
         <div class='row'>
                <p>Value</p>
                <textarea  bind:value={textAreaValue}></textarea>
                
         </div>
 

     
    </div>

    <div class='list-container'>
        {#each ListEntry  as  entry}
          <div class='attr-container'>
        <div class='row'>
         
            <button on:click={()=>remove(entry)}>Remove</button>
           <input type="text" value={entry.key}> 
           
        </div>
        <span> : </span>
         <div class='row'>
           
                <textarea  value={entry.value}></textarea>
                
         </div>
 

     
    </div>
        {/each}



    </div>

</div>
<style>
  .container{
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
  .attr-container{
      display:flex;
      justify-content: center;
      align-items: stretch;
  }
  input,textarea{
      font-size: 25px;
    
      resize: none;
  }
  input{
    text-align: center;
    width: 50%;
    padding: 12px;
    transform: translateY(50%);
    border-bottom-right-radius: 12px;
    border-top-right-radius: 12px;
    border: 1px solid #0000005e;
    border-left: none;
  }
  textarea{
      height: 100px;
      width: 90%;
      border-radius: 12px;
    box-shadow: 6px 3px 12px #00000069;
  }
  .row{
    position: relative;
    margin-top: 30px;
  }
  p{
    font-size: 20px;
    font-weight: bold;
    position: absolute;
    left: 50%;
    top: -30px;
  }
  span{
      font-size: 30px;
      margin:0 20px;
      margin-top: auto;
      margin-bottom: auto;
      margin-left: -3%;
      transform: translateY(30%);
  }
  .row:nth-of-type(1){
      flex-grow: 1;
  }
  .row:nth-of-type(2){
    flex-grow: 2;
    }
    button{
    min-width: 100px;
    font-size: 20px;
    padding: 15px;
    border-radius: 8px;
    border: 0px solid;
    border-bottom-right-radius: 0;
    border-top-right-radius: 0;
    margin-right: -13px;
    transform: translateY(calc(50% - 2px));
    
}
button:active{
    transform-origin: center bottom;
    transform: scale(0.9) translateY(calc(50% - 2px));
}
</style>