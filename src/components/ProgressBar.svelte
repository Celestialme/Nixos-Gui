<script lang='ts'>
export let title;
export let value;
export let max_value;
</script>

<div style='text-align:left;padding:3px 0'>
    <span class='title'>{title}</span>
    <div class='progress-track' >
        <div class='progress-bar' style='width:{value/max_value * 100}%'></div>
    </div>
</div>

<style>
    .progress-track{
        height: 10px;
        width: 100%;
        background: #e0e0e0;
        border-radius: 10px;
        margin-top: 10px;
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
</style>