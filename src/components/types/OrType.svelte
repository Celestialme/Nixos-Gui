<script lang='ts'>
import AttributeSetType from '@src/components/types/AttributeSetType.svelte';
import BooleanType from '@src/components/types/BooleanType.svelte';
import ListType from '@src/components/types/ListType.svelte';
import StringType from "@src/components/types/StringType.svelte";
import OneOfType from "@src/components/types/OneOfType.svelte";
import Default from './Default.svelte';
import {setContainerHeight} from '@src/utils/globalFunctions';
export let type='';
export let name;
let types = type.split(' or ')
let chosenType;
let selectedType='none'
let container:HTMLElement;
$:chosenType = getype(selectedType)

function getype(selectedType){
   container && (container.style.height = 'auto');
    switch (true) {
        case selectedType=='boolean':
        chosenType=BooleanType
            break;
        case selectedType=='null'||selectedType=='none':
            chosenType=''
            break
        case selectedType.startsWith('string')||selectedType.startsWith('Concatenated string')||selectedType.startsWith('path')||selectedType.includes('integer'):
            chosenType=StringType
        break
        case selectedType.startsWith('list of'):
            chosenType=ListType
            break;
        case selectedType.startsWith('attribute'):
            chosenType=AttributeSetType
            break
        case selectedType.startsWith('one of'):
            chosenType=OneOfType
            break
        default:
        chosenType=Default
            break;
    
    }
    return chosenType
}


</script>
<div class:container={selectedType=="none"} style="background: #F8F8F8;width:90vw;margin:auto" use:setContainerHeight bind:this={container}>
    
    <select bind:value={selectedType}>
    <option value="none" disabled hidden >Choose type</option>
    {#each types as curType }
    <option value={curType}>{curType.replace(/(one of).*/,'$1')}</option>
    {/each}
</select>

</div>
<svelte:component this={chosenType} {name} type={selectedType}/>


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
    .container{
    padding-top: 8px;
    overflow-y: auto;
    overflow-x: hidden;
    border-radius: 13px;
    border-top-left-radius: 0;
    border-top-right-radius: 0;
}
</style>

