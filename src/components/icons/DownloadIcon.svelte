<svg xmlns="http://www.w3.org/2000/svg" width="41" height="39" viewBox="0 0 20 20" on:click>
    
    <path style="fill:{marked? "red":""}" d="M17 12v5H3v-5H1v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5z"/>
    <path style="fill:{marked? "#03e703":""}" d="M10 15l5-6h-4V1H9v8H5l5 6z"/>
  </svg>

<script lang="ts">

 export let marked;
</script>


<style>
     svg{
         top:2px;
         right: 4px;
        
    }
    svg:active{
        transform: scale(0.8);
    }
</style>