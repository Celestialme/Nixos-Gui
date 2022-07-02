<script>
import { ast, changes, needsSaving } from "@src/store/store";

import {  find_key_value,setContainerHeight } from "@src/utils/globalFunctions";
export let name;
export let type



let _value =  $changes[name]?.nix || find_key_value($ast,name)[1];
if(_value){
_value=_value.replace(/\s*⇐change|\s*⇐ADD/g,'')
console.log(_value);
}
let types = type.replace('one of','').trim().split(/\s*,\s*/g)
let selectedType = _value||undefined;


function change(){

   $changes[name]={nix:selectedType}
   $needsSaving=true;
}
</script>
<div class="container" use:setContainerHeight>
    
    <select bind:value={selectedType} on:change={change}>
        <option value="none" disabled hidden >Choose Value</option>
        {#each types as curType }
        <option value={curType}>{curType}</option>
        {/each}
    </select>
</div>
<style>
    select{
        font-size: 25px;
        text-align: center;
        display: block;
        margin: auto;
        padding:10px;

    }
    option{
        padding: 10px;
    }
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
</style>