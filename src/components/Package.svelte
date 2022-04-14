<div class="container">

    <p class='name'>{name}</p>
    <p class='description'>{description}</p>
    <p class='version'>version: {version}</p>







</div>


<script>

    export let name;
    export let description;
    export let version ;

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
    .description,.version{
        font-size: 20px;
    }
    p{
        width:60%;
        margin:auto
    }
</style>