
 <div class='top-bar'>
    <div class="back-button" on:click={navigate}></div>
    <span>{name}</span>
     <SaveSettingsIcon/>
</div>
<p>Example: {example}</p>
<p>Type: {type}</p>
<script>
import { goto } from "$app/navigation";
import SaveSettingsIcon from "./icons/SaveSettingsIcon.svelte";
export let name;
export let example;
export let type;
        function navigate(){
        goto('/',{replaceState:true})
    }

</script>
<style>

.top-bar{
    background-color: #3e2e2e;
    font-size: 30px;
    text-align: center;
    color: white;
    padding: 20px;
    margin-bottom: 20px;
    
}

p{
    font-size: 25px;
    text-align: center;
}
p:last-of-type{
    margin-bottom: 20px;
}
.back-button{
    float: left;
    border: solid;
    color: white;
    width: 19px;
    height: 19px;
    border-width: 12px 0 0px 12px;
    transform: rotate(-45deg);
    z-index: 1;
    box-sizing: content-box;
    }
    .back-button:active{
      transform: scale(0.9) rotate(-45deg);
    }
</style>