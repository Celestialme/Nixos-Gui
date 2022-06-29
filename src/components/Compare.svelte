<script>
import { ast, markedPkgs } from "@src/store/store";

import { Ast2Text, config, getPkgs } from "@src/utils/globalFunctions";
import { invoke } from "@tauri-apps/api/tauri";
import axios from "axios";
import Denied from "./Denied.svelte";
export let compare;
let access=undefined;
let task ="Save configuration"
let conf = Ast2Text($ast).split(/\n/)
async  function save(){
    //TODO write to config file 
    let configuration = conf.join('\n').replace(/⇐change|⇐ADD/g,'')
    // let data = JSON.stringify(configuration)
    await invoke("save_config",{payload:configuration}).then(data=>access = data!="denied")
    await invoke("get_config").then(data=>$ast = JSON.parse(data)).then(()=>$markedPkgs=getPkgs($ast))
    access && (compare = false);
}
function cancel(){
    invoke("get_config").then(data=>$ast = JSON.parse(data)).then(()=>$markedPkgs=getPkgs($ast))
compare = false;
}
</script>
{#if access==false}
<Denied {task} bind:access/>
{/if}

<div class='container'>
    <div class='title'>

        <p>DOES IT LOOK GOOD?!</p>
        <button class='save' on:click={save}>SAVE</button>
        <button class='cancel' on:click={cancel}>Cancel</button>
    </div>

<div class='content'>

    {#each conf as line }
    <p 
    class:change={line.includes('⇐change')}
    class:add={line.includes('⇐ADD')}
    >{line}</p>
    {/each}
    
</div>


</div>

<style>
    .container{
    position: absolute;
    width: 99vw;
    height: 98vh;
    left: 10px;
    top: 1vh;
    color: black;
    font-size: 25px;
    padding: 0px 30px;
    background: #322f2f;
    box-shadow: 3px 4px 20px 3px black;
    color: #45ffe5;
    border-radius: 12px;
    overflow: hidden;
    }
    .content{
        height: calc(98vh - 78px);
        overflow: auto;
    }
    p{ margin: 6px 0;min-height: 20px; white-space: pre;}
    .add{
        background-color:#35ae21;
    color: white;
    padding: 10px;
    border-radius: 10px;
    }
    .change{
    background-color: #0790b4;
    color: white;
    padding: 10px;
    border-radius: 10px;
    }
    .title{
    text-align: center;
    background: #b4a3a321;
    margin-bottom: 30px;
    display:flex;
    justify-content: center;
    align-items: center;
  
    }
  button{ 
    padding: 20px 8px;
    background-color: #352e2e;
    padding: 20px;
    border: none;
    border-radius: 4px;
    color: white;
    font-size: 16px;
    margin: 4px 1px;
}
button:active{
transform: scale(0.9)
}
  .save{
      margin-left: auto;
  }
  .title p{ margin-left: auto;}
</style>