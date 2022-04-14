<script>
export let name;
</script>
<div class='container'>
    
  
    <textarea type="text" placeholder={'input value for: ' + name.split('.').slice(-1)[0]}/>
</div>

<style>
.container{
    text-align: center;
    height: 50vh;
}

textarea{
    width: 80%;
    height: 100%;
    text-align: center;
    border-radius: 12px;
    border: 1px solid;
    padding: 10px;
    margin: 10px 0;
    font-size: 30px;
    resize: none;
    
}

</style>