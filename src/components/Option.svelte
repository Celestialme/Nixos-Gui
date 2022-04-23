<div class="container" on:click={navigate}>

    <p class='name'>{name}</p>
    <p class='description'>Description: {@html description}</p>
    <p class='example'>Default: {JSON.stringify(defaultValue)}</p>
    <p class='example'>Example: {example}</p>
    <p class='type'>Type: {type}</p>
    {#if name.includes('<name>')}
    <p class='warning'> This is only template for option. you need to choose value for {name.split('<name>')[0].split('.').slice(-2)[0]} first OR create ONE</p>
    {/if}





</div>

<script>
import { goto } from "$app/navigation";



    export let name;
    export let description;
    export let example ;
     export let defaultValue ;
    export let type ;
    $:example =example?JSON.stringify(example).replace(/,/g,', '):example
    function navigate(){
        if(name.includes('<name>'))return
        let params =  new URLSearchParams({name,example,type})
        goto('/OptionEdit?'+params.toString())
    }


</script>

<style>
   .container{
    text-align: center;
    border-bottom: 1px solid;
    padding: 30px;
    margin: 10px;
    border-radius: 12px;
    box-shadow: 3px 2px 10px 1px #00000066;
    cursor: default;
    }
    .name{
        font-size: 30px;
    }
    .description,.example,.type{
        font-size: 20px;
    }
    p{
        width:60%;
        margin:auto
    }
    .warning{
        color:red;
        font-size: 20px;
    }
</style>