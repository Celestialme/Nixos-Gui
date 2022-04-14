<script>
import AttributeSetType from '@src/components/types/AttributeSetType.svelte';
import BooleanType from '@src/components/types/BooleanType.svelte';
import ListType from '@src/components/types/ListType.svelte';
import StringType from "@src/components/types/StringType.svelte";
import OneOfType from "@src/components/types/OneOfType.svelte";
import Default from './Default.svelte';
export let type='';
export let name;
let types = type.split(' or ')
let choseType;
let selectedType='none'

$:choseType = getype(selectedType)

function getype(selectedType){
   
    switch (true) {
        case selectedType=='boolean':
        choseType=BooleanType
            break;
        case selectedType=='null'||selectedType=='none':
            choseType=''
            break
        case selectedType.startsWith('string')||selectedType.startsWith('Concatenated string')||selectedType.startsWith('path')||selectedType.includes('integer'):
            choseType=StringType
        break
        case selectedType.startsWith('list of'):
            choseType=ListType
            break;
        case selectedType.startsWith('attribute'):
            choseType=AttributeSetType
            break
        case selectedType.startsWith('one of'):
            choseType=OneOfType
            break
        default:
        choseType=Default
            break;
    
    }
    return choseType
}


</script>

<select bind:value={selectedType}>
    <option value="none" disabled hidden >Choose type</option>
    {#each types as curType }
        <option value={curType}>{curType.replace(/(one of).*/,'$1')}</option>
    {/each}
</select>

<svelte:component this={choseType} {name} type={selectedType}/>


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

