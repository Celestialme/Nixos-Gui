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
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { onDestroy } from "svelte";
import Back from "./icons/Back.svelte";
import Home from "./icons/Home.svelte";
// import io from 'socket.io-client'
// let socket = io("https://exp2.celestialmee.repl.co")
let optionKeys;
let subMenus:any = [];
let worker = new Worker('worker.js');
let response;
$:worker.postMessage({type:'filterDict',payload: {dict:$optionList,filterKey:$OptionInputValue}})
 $:invoke("filter_dict",{filterKey:$OptionInputValue})

    let filter;
    // worker.onmessage=filter = async ({data})=>{
    //   response = data;
    //     if(data.type=='filterDict'){
    //       console.log(data.value)
    //       optionKeys =  Object.keys(data.value)
    //       subMenus= new Set();
    //       for (let subMenu of optionKeys) {
    //           let submenuSplit = subMenu.split('.');
    //           subMenus.add(submenuSplit[0]);
    //         }


    //         subMenus=[...subMenus]

    //     }else if(data.type=='filterDict-repl'){
            
    //        invoke("repl",{payload:`builtins.toJSON (builtins.attrNames config.${$OptionInputValue.slice(0,-1)})`})
         
    //     }
    // }


//fake fetching data with promise

let unlisten;
listen('repl', (e:any) => {
  let res = JSON.parse(JSON.parse(e.payload))
  let temp ={}
          for(let item of res){
            temp["<"+item+">"]=response.Value
          }
          console.log(62,temp)
          filter({payload:{Type:"filterDict",Value:temp}})
}).then(_unlisten=>unlisten=_unlisten)

let unlisten2;
listen('filterDict',filter = (e:any) => {
  
  response = e.payload
  if(e.payload.Type == "filterDict"){
    
  optionKeys =  Object.keys(e.payload.Value)
          subMenus= new Set();
          for (let subMenu of optionKeys) {
              let submenuSplit = subMenu.split('.');
              subMenus.add(submenuSplit[0]);
            }


            subMenus=[...subMenus]
  }else if(e.payload.Type=='filterDict-repl'){
            
                   invoke("repl",{payload:`builtins.toJSON (builtins.attrNames config.${$OptionInputValue.slice(0,-1)})`})
                 
                }
}).then(_unlisten=>unlisten2=_unlisten)

onDestroy(()=>{
    unlisten()
    unlisten2()
})

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