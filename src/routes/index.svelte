
    <div class='top-panel'>
    
        <Search bind:inputValue={inputValue} {keyUpFn}/>
        
    </div>
<div class="body">

    <div class='left-panel'>
        {#if $currentScreen==1}
     
            
        <SubMenus/>
    
        {:else}
        <button class:active={$currentScreen==0} on:click={()=>$currentScreen=0}>PACKAGES</button>
        <button class:active={$currentScreen==1} on:click={()=>$currentScreen=1}>OPTIONS</button>
        <button class:active={$currentScreen==2} on:click={()=>$currentScreen=2}>SYSTEM</button>
        <!-- <button class:active={$currentScreen==3} on:click={()=>invoke("get_nix_env_pkgs").then(data=>console.log(data))}>Update</button> -->
        {#if $needsSaving}
        <div class='controlls'>
           
            <button on:click={apply}> APPLY </button>
            <button on:click={discard}> DISCARD </button>
        </div>
        {/if}


        {/if}
    </div>
    <div class='right-panel'>
        
      <svelte:component inputValue={inputValue} bind:keyUpFn={keyUpFn} this={getCurrentScreen()} />
    </div>


    {#if compare}
    <Compare bind:compare={compare}/>
    {/if}

</div>


<script lang='ts'>
   import PackagesScreen from "@src/screens/PackagesScreen.svelte";
   // @ts-ignore
import OptionsScreen from "@src/screens/OptionsScreen.svelte";
import SystemScreen from "@src/screens/SystemScreen.svelte";
   import SubMenus from "@src/components/SubMenus.svelte";
   import Compare from "@src/components/Compare.svelte";
import { ast, changes, currentScreen, markedPkgs, needsSaving } from "@src/store/store";
import { findNode, getPkgs, setOption } from "@src/utils/globalFunctions";
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { onDestroy } from "svelte";
import axios from 'axios';
import Search from "@src/components/Search.svelte";
let inputValue:string = '';
let keyUpFn:Function=()=>{};


if(!$ast){
// axios.get('ast.json').then(data=>$ast = data.data).then(()=>$markedPkgs=getPkgs($ast))
    invoke("get_config").then((data:any)=>$ast = JSON.parse(data)).then(()=>$markedPkgs=getPkgs($ast))
    
}else{
    $markedPkgs=getPkgs($ast)
}

let compare = false;
   function getCurrentScreen(){
       switch ($currentScreen) {
           case 0:
               return PackagesScreen
           case 1:
               return OptionsScreen
           case 2:
               return SystemScreen 
         
       }
   }

   function apply(){
    const NODE_ATTR_SET = findNode($ast,"NODE_LAMBDA").findNode('self',"NODE_ATTR_SET")
    for(let key in $changes){
    setOption(NODE_ATTR_SET,key,$changes[key].nix)
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
 
    .top-panel{
    width: calc(100% - 300px);
    left: 300px;
    height: 60px;
    background: #524F4F;
    text-align: center;
    overflow: hidden;
    position: relative;
    background: linear-gradient(90deg, #504d4d 0%, #504d4d 31.03%, #4f4c4c 98.21%);
   
    }
     :global(*){
        margin:0;
        padding: 0;
        box-sizing: border-box;
    }
    :global(body){
        background: linear-gradient(180deg, #504c4c 0%, #504d4d 31.03%, #403F3F 98.21%);
        height: 100vh;
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
        background: linear-gradient(180deg, #4f4c4c 0%, #403F3F 31.03%, #403F3F 98.21%);
        padding-top:60px;
      
    }
    .left-panel button{
        display: block;
    width: 200px;
    margin: 10px auto;
    padding: 11px;
    padding-left:20px;
    font-size: 25px;
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
    padding-top: 8px;
    overflow-y: auto;
    flex-grow: 1;
    overflow-x: hidden;
    background: #F8F8F8;
    border-radius: 13px;
    margin-right: 15px;
    position: relative;
    }

    .active{
        background: #8e8e8e4d !important;
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





