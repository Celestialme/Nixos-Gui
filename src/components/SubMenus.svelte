<div class='container'>
  <div class='icons'>

    <Back/>
    
    <Home/>
  </div>
  {#each subMenus  as subMenu}
    <button on:click={()=>click(subMenu)}>{subMenu.replace(/^<|>$/g,'').split(/([A-Z]{2,})|(?=[A-Z])/) // split names by uppercase letters
      .join(' ')}</button>
  {/each}

</div>
<script lang='ts'>
import { optionList,OptionInputValue } from "@src/store/store";
import Back from "./icons/Back.svelte";
import Home from "./icons/Home.svelte";
// import io from 'socket.io-client'
// let socket = io("https://exp2.celestialmee.repl.co")
let optionKeys;
let subMenus:any = [];
let worker = new Worker('worker.js');
$:worker.postMessage({type:'filterDict',payload: {dict:$optionList,filterKey:$OptionInputValue}})

worker.onmessage = ({data})=>{
if(data.type!='filterDict')return
  optionKeys =  Object.keys(data.value)
  subMenus= new Set();
  for (let subMenu of optionKeys) {
      let submenuSplit = subMenu.split('.');
      subMenus.add(submenuSplit[0]);
    }


    subMenus=[...subMenus]

}







function click(subMenu){
  console.log($OptionInputValue)
  //@ts-ignore
  $OptionInputValue = $OptionInputValue.split('.').slice(0,-1).join('.')
  console.log($OptionInputValue)
  $OptionInputValue+='.'+subMenu+'.'
  $OptionInputValue = $OptionInputValue.replace(/^\./,'')
}




</script>

<style>
   button{
        display: block;
       width: 200px;
       margin: 10px auto;
       padding: 7px;
       font-size: 20px;
       border-radius: 6px;
       border:1px solid;
    }
    button:active{
        transform: scale(0.9) ;
    }
   
    .container{
      height: 100%;
      overflow-y: auto;
      padding-top: 10px;
      margin-bottom: 10px;
    }
 
    .icons{
      display: flex;
      justify-content: space-evenly;
      align-items: center;
    }
</style>