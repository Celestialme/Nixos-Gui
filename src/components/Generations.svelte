<div class="container">
    <button on:click={()=>state.value=state.none} class="close">X</button>
    
    <div class="table-wrapper">
        
    <table>
        <tr>
          <th>Name</th>
          <th>Url</th>
          <th></th>
        </tr>
        
        {#each generations as generation}
           <tr>
               <td>{generation.name}</td>
               <td>{generation.url}</td>
               <td><button>Remove</button> </td>
           </tr>
        {/each}
       
      </table>
    </div>
    
    
    
    </div>
    
    
    <script lang='ts'>
    export let state;
    
    let generations=[
        {name:"Nixos",url:"https://github.com/default"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
        {name:"Nixos-unstable",url:"https://github.com/unstable"},
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