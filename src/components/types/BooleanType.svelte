<script lang='ts'>



let state=false;
let switchElement:HTMLElement;
function toggle(){
    state=!state
    
    switchElement.style.transform= state? "translate(calc(-100% - 5px))":"none"
}
</script>
<div class='container' style="text-align: center;">
    
    <div class='switch-container'>
        <div class='switch' bind:this={switchElement}>
            <button on:click={toggle}>Enable</button>
            <button on:click={toggle}>Disable</button>
        </div>
    </div>



</div>
<style>
*{
    box-sizing: border-box;
}
.switch-container{
    overflow: hidden;
    width: 200px;
    display: inline-block;
    border: 1px solid;
}
.switch{
    transition: 0.5s transform;
    white-space: nowrap;
    /* transform: translate(-100%); */
}
button{
    padding: 10px;
    width:calc(100% + 5px);
    margin:-2px
}

</style>