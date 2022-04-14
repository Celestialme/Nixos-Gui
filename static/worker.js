function filterPackages({packages,keys,value}){
    let  filteredKey= keys.filter((key)=>{
   
   return packages[key].pname.includes(value) || packages[key].description.includes(value)
})


filteredKey = filteredKey.sort((a,b)=>{
 let temp = (packages[a]['pname'].startsWith(value)?0:1) - (packages[b]['pname'].startsWith(value)?0:1)

 if(temp==0){
     return packages[a]['pname'].length-packages[b]['pname'].length
 }else{
     return temp
 }

})

postMessage({type:'filterPackages',value:filteredKey.slice(0,50)})
}














function filterOptions({packages,keys,value}){
    value = value.replace(/\.$/, '');
    let  filteredKey= keys.filter((key)=>{
   
   return key.includes(value)
})

filteredKey = filteredKey.sort((a,b)=>{
 let temp = (a.startsWith(value)?0:1) - (b.startsWith(value)?0:1)

 if(temp==0){
     return a.length-b.length
 }else{
     return temp
 }

})


postMessage({type:'filterOptions',value:filteredKey.slice(0,50)})
}





   function filterDict({dict,filterKey}){
    
        let temp={}
    for(let key in dict){
     if(filterKey !='' && key.startsWith(filterKey))   {
        let tempKey;
 
        if(key.split('.').indexOf(filterKey.match(/\.\w+$/g)?.[0])!=-1){
             tempKey = key.replace(new RegExp(filterKey+'\\.?'),'')



        }else{
            let tempFilterKey=filterKey.match(/^.*\./)?.[0]??''
            tempKey = key.replace(new RegExp(tempFilterKey+'\\.?'),'')
        }

        if(tempKey!=''){
            temp[tempKey]=dict[key];
        }

     }else if(filterKey==''){
         temp[key] = dict[key]
     }

    }




postMessage({type:'filterDict',value:temp})
    }




onmessage = function({data}){
    
    if(data.payload.value ==''){
        postMessage([])
        return
    }
    switch (data.type) {
        case 'filterPackages':
            filterPackages(data.payload)
            break;
        case 'filterOptions':
            filterOptions(data.payload)
        case 'filterDict':
            filterDict(data.payload)
        default:
            break;
    }
   
}

