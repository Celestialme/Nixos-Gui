<div>
<button on:click={()=>access=undefined}>X</button>
<p>
you have to be root to do this opperation or use SUDO
</p>

</div>
<script>
    export let access;
</script>
<style>
    div{
        position: absolute;
        left: 50%;
        top: 30%;
        border: 1px solid black;
        padding: 100px;
        z-index: 111;
        color: red;
        background: lightgoldenrodyellow;
        transform: translate(-50%);
    

    }
    p{
        text-align:center;
    }
    button{
        float:right
    }
</style>