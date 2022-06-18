<div>
<button on:click={()=>access=undefined}>X</button>
<p>
you have to be root to {task} or use SUDO
</p>

</div>
<script>
    export let access;
    export let task;
</script>
<style>
    div{
        position: absolute;
    padding: 100px 50px;
    top: 50%;
    left: 50%;
    width: 50%;
    min-width: 800px;
    text-align: center;
    background: #403f3f;
    transform: translate(-50% ,-50%);
    border: 1px solid;
    border-radius: 12px;
    color: white;
    z-index:1

    }
    p{
        text-align:center;
    }
    button{
        position: absolute;
        right: 0;
        top: 0;
        font-size: 25px;
        padding: 10px 20px;
        border: none;
        border-radius: 5px;
        margin: 3px;
    }
</style>