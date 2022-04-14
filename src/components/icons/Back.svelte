<div class='back-button' on:click={click}></div>

<style>
       .back-button{
      display: inline-block;
    border: solid;
    color: white;
    width: 10px;
    height: 10px;
    border-width: 10px 0 0px 10px;
    transform: rotate(-45deg);
      z-index: 1;
      box-sizing: content-box;
    }
    .back-button:active{
      transform: scale(0.9) rotate(-45deg);
    }
</style>


<script>
import { OptionInputValue } from "@src/store/store";


    function click(){
        if ($OptionInputValue == '') return;
      let temp =  $OptionInputValue.split('.')
      if(temp[temp.length-1]=='') temp.splice(-1)
      temp.splice(-1)
      $OptionInputValue=(temp.join('.')+'.').replace(/^\.$/,'')//if only '.' left after delete
    }
</script>