

 Array.prototype.last = function(){
return this[this.length-1]
    
}
 
  Array.prototype.removeLast = function(){
 this.splice(-1)
    return this
}
 
 
 
 
 
 
 
 

export function findNode(node,key,text?,found={value:false}):any {
    node = node=='self'?this:node //is it chained or passed


    if(node.kind==key){
        if(text){
            if(Ast2Text(node) !=text)return
            found.value =node
            found.value.findNode = findNode
            return
        }
        found.value =node
        found.value.findNode = findNode
        return 
    }
    
    
    
    
    if(found.value ){
    found.value.findNode = findNode
    }else if(node.children){
        for(let child of node.children){
        findNode(child,key,text,found)
    }
    }
    return found.value
}



export function removeLastChar(char,string):string{
string = string.split('')

for (let i = string.length; i >= 0; i--) {
    if(string[i] == char){
        string[i]='';
        break
    }
    
}

return string.join('')
}


export function find_key_value(node,text,returnType?:'node'|'',found={value:false}):any{
    node = node=='self'?this:node
if(node.kind=='NODE_KEY_VALUE'){
    const KEY_VALUE= removeLastChar(';', Ast2Text(node).replace('=','=&') ).split(/\s*=&\s*/)
    
    if(KEY_VALUE[0]==text){
        found.value =returnType == 'node' ?node: KEY_VALUE
        return
    }
    
}
if(!node.children){
        return 
    }
for(let child of node.children){
    find_key_value(child,text,returnType,found)
}
if(found.value &&  returnType == 'node'){
found.value.findNode = findNode
found.value.find_key_value = find_key_value
}
return found.value
}








let textArray = [];
export function Ast2Text(node,text={value:''}){

if(node.text){
    text.value += node.text
    textArray.push(node.text)
}
if(!node.children){
    return text.value
}

for(let child of node.children){
    Ast2Text(child,text)
}

return text.value
}


export function setOption(node,key,value,main=true,replaced={value:false}){
if(node.kind=='NODE_KEY_VALUE' && node.children){
  const KEY_VALUE= Ast2Text(node).replace('=','=&').split(/\s*=&\s*/);
  if(KEY_VALUE[0].trim()==key){ // replace if exists
 
                
        // node.text = key+' = '+value +';  #changed by NIX_GUI'
        for (var member in node) delete node[member]; //clear object without losing reference.
       node.children = [{kind: 'NODE_VALUE',text:key +' = '+value+';  ???change'}]
       node.kind = 'NODE_KEY_VALUE'
        replaced.value=true
        return 
      
  }

}

if(!node.children){
    return
}
for(let child of node.children){
    setOption(child,key,value,false,replaced)
}
if(!main || replaced.value)return

let endOfSection = findLastIndex( node.children,child => child.kind=='TOKEN_COMMENT'&&child.text=='#END OF NIX_GUI SECTION')
    if(endOfSection){ //section exists
     
        node.children.splice(endOfSection,0,{kind: 'NODE_KEY_VALUE',children:[{kind:'NODE_VALUE', text:key +' = '+value+'; ???ADD' }]})
        node.children.splice(endOfSection+1,0,{kind: 'TOKEN_WHITESPACE',text:"\n"})
       
    }else{
        let lastIndex =findLastIndex( node.children,child => child.kind=='NODE_KEY_VALUE')
        
        node.children.splice( lastIndex+2  ,0,{kind: 'TOKEN_COMMENT',text:"\n\n\n#START OF NIX_GUI SECTION\n\n\n"})
        node.children.splice( lastIndex+3 ,0,{kind: 'NODE_KEY_VALUE',children:[{kind:'NODE_VALUE', text:key +' = '+value+'; ???ADD' }]})
         node.children.splice( lastIndex+4  ,0,{kind: 'TOKEN_WHITESPACE',text:"\n"})
         node.children.splice( lastIndex+5  ,0,{kind: 'TOKEN_COMMENT',text:"#END OF NIX_GUI SECTION"})
         node.children.splice( lastIndex+6  ,0,{kind: 'TOKEN_WHITESPACE',text:"\n\n\n\n\n\n\n\n"})
        
    }



}

export function addPkg(pkg,ast){
    pkg = pkg.replace(/^nixos\./,"")
let pkgsNode = find_key_value(ast,'environment.systemPackages','node' ).findNode('self','NODE_WITH').findNode('self','NODE_LIST')
pkgsNode.children.splice(-2,0,{kind:'WHITE_SPACE',text:'\n     '})
pkgsNode.children.splice(-2,0,{kind:'NODE_IDENT',text:pkg+' ???ADD'})

}
export function removePkg(pkg,ast){
    pkg = pkg.replace(/^nixos\./,"")
    let pkgsNode = find_key_value(ast,'environment.systemPackages','node' ).findNode('self','NODE_WITH').findNode('self','NODE_LIST')
   let index =  findLastIndex(pkgsNode.children,(child)=>{
       debugger
        return Ast2Text(child).replace(' ???ADD','')==pkg;
    })
    pkgsNode.children.splice(index-1,2)
}

export function getPkgs(ast){
let pkgsNode = find_key_value(ast,'environment.systemPackages','node' ).findNode('self','NODE_WITH').findNode('self','NODE_LIST')
let pkgs = []
    for(let child of pkgsNode.children){
        if(child.kind === 'NODE_IDENT'){
            pkgs.push("nixos."+Ast2Text(child).replace(' ???ADD',''))
        }
    }
return pkgs
}

function findLastIndex(arr,fn){
    let index=-1
for(let i=0;i<arr.length;i++){
 if(fn(arr[i])){
     index=i
 }

}
return index==-1 ? false : index

}


  export function getKeyName(key){
    return key.replace(/^nixos\./,"")
    }
   


export     const config = {
  headers: {
    'Content-Type': 'application/json'
  },
//  transformRequest: [function (data) {
// 	var temp=''
//     Object.entries(data).forEach(function(key_value) {
//              temp += key_value[0]+'='+key_value[1]+'&'
//           });
// 	data = temp
//     return data;
//   }],
//   transformResponse: [function (data) {

//     return JSON.parse(data).data;
//   }],
}

// get top bar height
export function setContainerHeight(node){
    let topBarHeight = document.getElementById('top-bar').offsetHeight+55
    node.style.height = "calc(100vh - "+topBarHeight+"px)"
}