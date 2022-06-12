<div class="container">
    <button on:click={()=>state.value=state.none} class="close">X</button>
    
    <div class="table-wrapper">
        
    <table>
        <tr>
          <th>INDEX</th>
          <th>DATE</th>
          <th>Current</th>
          <th></th>
        </tr>
        
        {#each generations as generation}
           <tr>
               <td>{generation.index}</td>
               <td>{generation.date}</td>
               <td>{generation.current?"✔️":""}</td>
               <!-- <td><button>Remove</button> </td> -->
           </tr>
        {/each}
       
      </table>
    </div>
    
    
    
    </div>
    
    
    <script lang='ts'>
    export let state;
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';

onMount(async ()=>{
    generations= await invoke("get_generations")
    generations = generations.map((item)=>
    {
        let temp={index:undefined,date:undefined,current:false,}
        if(item.includes('(current)')){
            item = item.replace("(current)","").trim()
            temp.current = true
        }
        item = item.split(/\s+/)
        temp.index=item[0]
        temp.date=item[1] +" "+item[2]
        console.log(temp)
        return temp
    })
    console.log(await invoke("get_generations"))
})
    let generations:any=[
 
    ]
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
   
    </style>