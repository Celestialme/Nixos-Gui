<script>
import { ast, changes, needsSaving } from "@src/store/store";

import { find_key_value ,getKeyName} from "@src/utils/globalFunctions";
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
<div class='container'>
    
  
    <textarea type="text" placeholder={'input value for: ' + getKeyName(name)} bind:value={value}  on:change={change}/>
</div>

<style>
.container{
    text-align: center;
    height: 50vh;
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