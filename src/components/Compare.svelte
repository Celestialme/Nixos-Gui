<script>
import { ast } from "@src/store/store";

import { Ast2Text } from "@src/utils/globalFunctions";
export let compare;

let conf = Ast2Text($ast).split(/\n/)

function save(){
    //TODO write to config file conf.join('\n').replace(/⇐change|⇐ADD/g,'')
    compare = false;
}
function cancel(){
    // TODO regenerate AST from RNIX
compare = false;
}
</script>


<div class='container'>
    <div class='title'>

        <p>DOES IT LOOK GOOD?!</p>
        <button class='save' on:click={save}>SAVE</button>
        <button class='cancel' on:click={cancel}>Cancel</button>
    </div>
{#each conf as line }
    <p 
    class:change={line.includes('⇐change')}
    class:add={line.includes('⇐ADD')}
    >{line}</p>
{/each}



</div>

<style>
    .container{
    position: absolute;
    width: calc(90% - 330px);
    height: 80%;
    left: 400px;
    top: 10%;
    color: black;
    font-size: 25px;
    padding: 20px 30px;
    overflow: auto;
    background: #322f2f;
    box-shadow: 3px 4px 20px 3px black;
    color: #45ffe5;
    border-radius: 12px;
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