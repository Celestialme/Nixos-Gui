<div class="container">
<button on:click={()=>state.value=state.none} class="close">X</button>

<div class="table-wrapper">
    
<table>
    <tr>
      <th>Name</th>
      <th>URL</th>
      <th></th>
    </tr>
    
    {#each channels as channel}
       <tr>
           <td>{channel.name}</td>
           <td>{channel.url}</td>
           <td><button on:click={()=>remove_channel(channel.name)}>Remove</button> </td>
       </tr>
    {/each}
   
  </table>
</div>


  <div class="add-new">
    <input type="text" placeholder ="NAME" class="name" bind:value={name} ><input type="text" placeholder="URL" class="url" bind:value={url}>
    <button on:click={add_channel}>ADD</button>
  </div>
</div>


<script lang='ts'>
export let state;
let name="";
let url="";
import { invoke } from '@tauri-apps/api/tauri'
import { onMount } from 'svelte';
let channels:any=[]
onMount(async ()=>{
    channels= await invoke("get_channels")
    channels = channels.map((item)=>JSON.parse(item))
    
})
function add_channel(){
    invoke("add_channel",{name,url}).then((data:Array<string>)=>{channels=data.map((item)=>JSON.parse(item))})
}
function remove_channel(name){
    invoke("remove_channel",{name}).then((data:Array<string>)=>{channels=data.map((item)=>JSON.parse(item))})
}
</script>


<style>
    .close{
        position: absolute;
        right: 0;
        top: 0;
        font-size: 25px;
        padding: 10px 20px;
        border: none;
        border-radius: 5px;
        margin: 3px;
    }
.container{
    position: absolute;
    padding-top: 100px;
    top: 50%;
    left: 50%;
    width: 50%;
    min-width: 800px;
    text-align: center;
    background: #403f3f;
    height: 50%;
    min-height: 550px;
    transform: translate(-50% ,-50%);
    border: 1px solid;
    border-radius: 12px;
    color: white;
}
th{

    padding-bottom: 20px;
}
td{
    padding-bottom:10px;
}
.table-wrapper{
    overflow: auto;
    max-height: calc(100% - min(50%,550px) + 100px);
}
table{
    width:100%;
    font-size: 25px;
    max-height: calc(100% - min(50%,550px));
    overflow-y:auto
}
table button{
    font-size:20px;
    padding:10px 15px;
    border-radius: 6px;
}
button:active{
    transform: scale(0.9);
}
input{
    font-size: 20px;
    padding: 5px
}
.name{
    width:170px
}
.url{
    margin-top: auto;
    width:300px;
}
.add-new{
    position: absolute;
    bottom: 0;
    text-align: center;
    width: 100%;
    height: 50px;
}
.add-new button{
    font-size: 20px;
    padding: 7px 35px;
    border-top-right-radius: 8px;
    border-bottom-right-radius: 8px;
}
</style>