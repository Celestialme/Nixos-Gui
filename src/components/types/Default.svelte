<script>
import { ast, changes, needsSaving } from "@src/store/store";

import { find_key_value ,getKeyName,setContainerHeight} from "@src/utils/globalFunctions";
export let name;

let value;
let _value = $changes[name] || find_key_value($ast,name)[1];
if(_value){
    value=_value.replace(/⇐.*$/,'')
}
function change(){
    $changes[name] = value;
    $needsSaving=true;
}

</script>
<div class='container' use:setContainerHeight>
    
  
    <textarea type="text" placeholder={'input value for: ' + getKeyName(name)} bind:value={value}  on:change={change}/>
</div>

<style>
.container{
    position: relative;
    text-align: center;
    width:90vw;
    padding-top: 8px;
    overflow:hidden;
    background: #F8F8F8;
    border-radius: 13px;
    margin:auto;
    border-top-left-radius: 0;
    border-top-right-radius: 0;
}

textarea{
    width: 80%;
    height: 100%;
    text-align: center;
    border-radius: 12px;
    border: 1px solid;
    padding: 10px;
    margin: 10px 0;
    font-size: 30px;
    resize: none;
    
}

</style>