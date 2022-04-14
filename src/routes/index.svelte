<p>Gui Package Manager</p>
<div class="body">
    
    <div class='left-panel'>
        {#if $currentScreen==1}
     
            
        <SubMenus/>
    
        {:else}
        <button class:active={$currentScreen==0} on:click={()=>$currentScreen=0}>Packages</button>
        <button class:active={$currentScreen==1} on:click={()=>$currentScreen=1}>Options</button>
        <button class:active={$currentScreen==2} on:click={()=>$currentScreen=2}>Shortcuts</button>
        {/if}
    </div>
    <div class='right-panel'>
        
      <svelte:component this={getCurrentScreen()} />
    </div>


</div>




<script>
   import PackagesScreen from "@src/screens/PackagesScreen.svelte";
   // @ts-ignore
import OptionsScreen from "@src/screens/OptionsScreen.svelte";
   import SubMenus from "@src/components/SubMenus.svelte";
import { currentScreen, OptionInputValue, optionList } from "@src/store/store";

   function getCurrentScreen(){
       switch ($currentScreen) {
           case 0:
               return PackagesScreen
           case 1:
               return OptionsScreen
         
       }
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
</style>





