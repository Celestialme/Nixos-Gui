<p>Gui Package Manager</p>
<div class="body">
    
    <div class='left-panel'>
        {#if $currentScreen==1}
     
            
        <SubMenus/>
    
        {:else}
        <button class:active={$currentScreen==0} on:click={()=>$currentScreen=0}>Packages</button>
        <button class:active={$currentScreen==1} on:click={()=>$currentScreen=1}>Options</button>
        <button class:active={$currentScreen==2} on:click={()=>$currentScreen=2}>Shortcuts</button>

        {#if $needsSaving}
        <div class='controlls'>
           
            <button on:click={apply}> APPLY </button>
            <button on:click={discard}> DISCARD </button>
        </div>
        {/if}


        {/if}
    </div>
    <div class='right-panel'>
        
      <svelte:component this={getCurrentScreen()} />
    </div>


    {#if compare}
    <Compare bind:compare={compare}/>
    {/if}

</div>




<script>
   import PackagesScreen from "@src/screens/PackagesScreen.svelte";
   // @ts-ignore
import OptionsScreen from "@src/screens/OptionsScreen.svelte";
   import SubMenus from "@src/components/SubMenus.svelte";
   import Compare from "@src/components/Compare.svelte";
import { ast, changes, currentScreen, installedPkgs, needsSaving } from "@src/store/store";
import { findNode, getPkgs, setOption } from "@src/utils/globalFunctions";
import axios from 'axios';
if(!$ast){
axios.get('ast.json').then(data=>$ast = data.data).then(()=>$installedPkgs=getPkgs($ast))
}else{
    $installedPkgs=getPkgs($ast)
}

let compare = false;
   function getCurrentScreen(){
       switch ($currentScreen) {
           case 0:
               return PackagesScreen
           case 1:
               return OptionsScreen
         
       }
   }

   function apply(){
    const NODE_ATTR_SET = findNode($ast,"NODE_LAMBDA").findNode('self',"NODE_ATTR_SET")
    for(let key in $changes){
    setOption(NODE_ATTR_SET,key,$changes[key])
    }
    $changes={}
    $needsSaving=false
    compare = true;
   }
   function discard(){
    $changes={}
    $needsSaving=false
   }
</script>




<style>
     :global(*){
        margin:0;
        padding: 0;
        box-sizing: border-box;
    }
    .body{
        width: 100%;
        height: calc(100vh - 70px );
        display: flex;

    }
    .left-panel{
        position: relative;
        min-width:300px;
        background-color: #000000d1;
      
    }
    .left-panel button{
        display: block;
    width: 200px;
    margin: 10px auto;
    padding: 11px;
    font-size: 20px;
    border-radius: 7px;
    width: 96%;
    text-align: left;
    background: transparent;
    color: white;
    border: none;
    }
    .left-panel button:active{
        transform: scale(0.9);
    }
    .right-panel{
        padding-top: 10px;
        overflow-y: auto;
        flex-grow: 1;
        overflow-x:hidden;
    }

    .active{
        background: #8e8e8e4d !important;
    }
   p{
    background: #3e2e2e;
    text-align: center;
    font-size: 30px;
    color: white;
    border-bottom: 1px solid white;
    padding: 15px;
    font-family: fantasy;
    padding-left: 15%;
   }
   .controlls{
       position: absolute;
       bottom:0;
       text-align: center;
       width:100%;
   }
   .controlls button{
    display: inline-block;
    width: calc(50% - 15px );
    text-align: center;
    background: #ff2d00;
    border-radius: 6px;
   }
   .controlls button:nth-of-type(1){
    background: #01cf0180;
   }
</style>





