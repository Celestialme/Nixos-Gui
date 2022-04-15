<script>
import {page} from '$app/stores'
import TopBar from '@src/components/TopBar.svelte';
import AttributeSetType from '@src/components/types/AttributeSetType.svelte';
import BooleanType from '@src/components/types/BooleanType.svelte';
import Default from '@src/components/types/Default.svelte';
import ListType from '@src/components/types/ListType.svelte';
import OneOfType from '@src/components/types/OneOfType.svelte';
import OrType from '@src/components/types/OrType.svelte';
import StringType from "@src/components/types/StringType.svelte";

let {name,example,type} = Object.fromEntries($page.url.searchParams)
example = example?example.replace(/,/g,', '):example
let choseType;


switch (true) {
    case type.includes(' or '):
        choseType=OrType
        break
    case type=='boolean':
        choseType=BooleanType
        break;
    case type.startsWith('string')||type.startsWith('Concatenated string')||type.startsWith('path')||type.includes('integer'):
        choseType=StringType
        break
    case type.startsWith('list of'):
        choseType=ListType
        break;
    case type.startsWith('attribute'):
        choseType=AttributeSetType
        break
    case type.startsWith('one of'):
        choseType=OneOfType
        break
    default:
    choseType=Default
        break;
}









</script>
<TopBar {name} {example} {type}/>
<svelte:component this={choseType} {name} {type} />

