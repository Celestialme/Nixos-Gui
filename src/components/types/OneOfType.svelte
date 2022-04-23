<script>
import { ast, changes, needsSaving } from "@src/store/store";

import {  find_key_value } from "@src/utils/globalFunctions";
export let name;
export let type



let _value =  $changes[name] || find_key_value($ast,name)[1];
if(_value){
_value=_value.replace(/\s*⇐change|\s*⇐ADD/g,'')
console.log(_value);
}
let types = type.replace('one of','').trim().split(/\s*,\s*/g)
let selectedType = _value||undefined;


function change(){

   $changes[name]=selectedType
   $needsSaving=true;
}
</script>

<select bind:value={selectedType} on:change={change}>
    <option value="none" disabled hidden >Choose Value</option>
    {#each types as curType }
        <option value={curType}>{curType}</option>
    {/each}
</select>
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
</style>