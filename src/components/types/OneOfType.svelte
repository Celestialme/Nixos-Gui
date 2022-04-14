<script>
export let type
let types = type.replace('one of','').trim().split(',')
let selectedType

</script>

<select bind:value={selectedType}>
    <option value="none" disabled hidden >Choose type</option>
    {#each types as curType }
        <option value={curType}>{curType}</option>
    {/each}
</select>
<style>
    select{
        font-size: 25px;
        text-align: center;
        display: block;
        margin: auto;
        padding:10px;

    }
    option{
        padding: 10px;
    }
</style>