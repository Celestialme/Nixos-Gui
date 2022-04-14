<script>


let inputValue=''
let ListEntry=[];


function add(){
    if(!inputValue) return
    ListEntry.push({value:inputValue})
    ListEntry=ListEntry
    
    inputValue=''
}
function remove(entry){
    ListEntry = ListEntry.filter((x)=>x!=entry)
}
</script>
<div class='container'>

    <div>
        <input type="text" placeholder={'ADD Entry'} bind:value={inputValue}>
        <button on:click={add}>ADD</button>
    </div>

    <div class='list-container'>
        {#each ListEntry  as  entry}
        <div>
            <input type="text" value={entry.value}>
            <button on:click={()=>remove(entry)}>Remove</button>
        </div>
        {/each}



    </div>









</div>

<style>
.container{
    text-align: center;
}

input{
    width: 30%;
    text-align: center;
    border-radius: 8px;
    border: 1px solid;
    padding: 10px;
    margin: 10px 0;
    font-size: 20px;
    border-bottom-right-radius:0 ;
    border-top-right-radius: 0
}
button{
    min-width: 100px;
    font-size: 20px;
    padding: 10px;
    border-radius: 8px;
    border:1px solid;
    border-bottom-left-radius:0 ;
    border-top-left-radius: 0;
    margin-left: -3px;
    
}
button:active{
    transform: scale(0.9);
}
</style>