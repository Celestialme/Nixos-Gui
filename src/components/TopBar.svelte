
<div id="top-bar">

    <div class='top-bar'>
        <div class="back-button" on:click={navigate}></div>
        <span>{name}</span>
        
    </div>
    <div class="info">
        <p>Example: {example}</p>
        <p>Type: {type}</p>
    </div>
</div>
<script>
import { goto } from "$app/navigation";
export let name;
export let example;
export let type;
        function navigate(){
        goto('/',{replaceState:true})
    }

</script>
<style>

.top-bar{
    background: linear-gradient(90deg, #504d4d 0%, #504d4d 31.03%, #4f4c4c 98.21%);
    font-size: 30px;
    text-align: center;
    color: white;
    padding: 20px;
    margin-bottom: 20px;
    
}
.info{
    width:90vw;
    margin:auto;
    border-top-left-radius: 13px;
    border-top-right-radius: 13px;
    background:#F8F8F8;
}
p{
    font-size: 25px;
    text-align: center;
}
p:first-of-type{
    padding-top: 20px;
}
p:last-of-type{
    padding-bottom: 20px;
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