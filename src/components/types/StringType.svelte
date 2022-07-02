<script>
import { ast, changes, needsSaving } from "@src/store/store";

import { find_key_value, getPkgs ,getKeyName,setContainerHeight} from "@src/utils/globalFunctions";
export let name;
let value;
let _value = $changes[name]?.nix || find_key_value($ast,name)[1];
if(_value){
    value=_value.replace(/⇐.*$/,'')
}



console.log(getPkgs($ast))



function change(){
    $changes[name] = {nix:value};
    $needsSaving=true;
}
</script>
<div class='container' use:setContainerHeight>
    
  
    <input type="text" placeholder={'input value for: ' + getKeyName(name)} bind:value={value} on:change={change}>
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
    width: 50%;
    text-align: center;
    border-radius: 12px;
    border: 1px solid;
    padding: 10px;
    margin: 10px 0;
    font-size: 30px;
}

</style>