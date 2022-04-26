//@ts-check
function getKeyName(key){
 
    for(var i=key.length-1;i>=0;i--){
        if(key[i]=='.'){
            i++
            break}
    }
    return key.slice(-(key.length - i))
}

function filterPackages({packages,keys,value}){
    let  filteredKey= keys.filter((key)=>{
   
   return key.includes(value) || packages[key].pname.includes(value) || packages[key].description.includes(value)
})

filteredKey = filteredKey.sort((a,b)=>{
 let byPname = (packages[a]['pname'].startsWith(value)?0:1) - (packages[b]['pname'].startsWith(value)?0:1) // sort  by pname
 let keyA = getKeyName(a)
 let keyB = getKeyName(b)
 let byKeyName = (keyA.includes(value)?0:1) - (keyB.includes(value)?0:1)
 if(byKeyName!=0){
     return byKeyName
 }else if(a.includes(value)){ // if both key includes value, place first the one which starts with value
     return (keyA.startsWith(value)?0:1) - (keyB.startsWith(value)?0:1) || (keyA.length - keyB.length)
 }
 else if(byPname==0){
     return packages[a]['pname'].length-packages[b]['pname'].length
 }else{
     return byPname
 }

})

postMessage({type:'filterPackages',value:filteredKey.slice(0,50)})
}













function filterOptions({packages,keys,value}){
     value = value.replace(/\.$/, '')
    
    let  filteredKey= keys.filter((key)=>{
   return key.includes(value.replace(/<.*>/g,'<name>'))
})
let matches = value.match(/<.*>/)
if(matches){
filteredKey = filteredKey.map((key)=>{
   return key.replace(/<.*>/g,matches[0])
})
}

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
        if(filterKey !='' && key.startsWith(filterKey) && !key.includes('<name>'))   {
            let tempKey;
    
            if(key.split('.').indexOf(getKeyName(filterKey))!=-1){
                tempKey = key.replace(new RegExp(filterKey+'\\.?'),'')



            }else{
                let tempFilterKey=filterKey.match(/^.*\./)?.[0]??''
                tempKey = key.replace(new RegExp(tempFilterKey+'\\.?'),'')
            }

            if(tempKey!=''){
                temp[tempKey]=dict[key];
            }

        }else if(filterKey !='' && key.startsWith(filterKey) && key.replace(filterKey,'').startsWith('<name>')) {
         
            
             temp['<celestialme>']=dict[key];
             temp['<celestialme2>']=dict[key];
             temp['<celestialme3>']=dict[key];
        }
        
        else if(filterKey==''){
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

