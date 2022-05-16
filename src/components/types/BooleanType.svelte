<script lang='ts'>
import { ast, changes, needsSaving } from "@src/store/store";

import { find_key_value,setContainerHeight} from "@src/utils/globalFunctions";
export let name;
let state=false;
let _value = $changes[name]?.toString() || find_key_value($ast,name)[1];
if(_value){
    state=JSON.parse(_value.replace(/⇐.*$/,''))
   
}



function toggle(){
    state=!state
    $changes[name]=state
    $needsSaving=true;
}

</script>
<div class='container' style="text-align: center;" use:setContainerHeight>
    
    <div class='switch-container'>
        <div class='switch' class:enabled={state}>
            <button on:click={toggle}>Enabled</button>
            <button on:click={toggle}>Disabled</button>
        </div>
    </div>



</div>

<style>
*{
    box-sizing: border-box;
}
.switch-container{
    overflow: hidden;
    width: 200px;
    display: inline-block;
    border: 1px solid;
}
.switch{
    transition: 0.5s transform;
    white-space: nowrap;
   transform: translate(calc(-100% - 5px));;
}
button{
    padding: 10px;
    width:calc(100% + 5px);
    margin:-2px
}
.enabled{
    transform: translate(0);
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