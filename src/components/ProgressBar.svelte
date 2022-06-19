<script lang='ts'>
export let value;
export let max_value;
export let success;
export let msg;
export let title="Downloading...";
export let wide = false;
</script>

<div style='text-align:left;padding:20px 0px' class:wide={wide}>
    <span class='title'>{success==undefined?title:success=="true"?"Finished Successfully":"Build Failed!"}</span>
    <div class='progress-track' >
        <div class='progress-bar' class:success={success=="true"} class:error={success=="false"} style='width:{value/max_value * 100}%'></div>
    </div>
    <span class='msg'>{@html msg}</span>
</div>

<style>
    .wide{
        width:calc(80vw - 300px)
    }
    .progress-track{
        height: 10px;
        background: #e0e0e0;
        border-radius: 10px;
        margin-top: 15px;
    }
    .progress-bar{
        width:0;
        height: 100%;
        background: #00bcd4;
        border-radius: 10px;
        transition:1s;
    }
    .title{
        font-size: 20px;
    }
    .success{
        background: green;
    }
    .error{
        background: red;
    }
</style>