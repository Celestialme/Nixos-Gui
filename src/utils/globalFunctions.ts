

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
        return
    }
    found.value =node
    return 
}
if(!node.children){
    return 
}

for(let child of node.children){
    findNode(child,key,text,found)
}
if(found.value){
found.value.findNode = findNode
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
       node.children = [{kind: 'NODE_VALUE',text:key +' = '+value+';  ⇐change'}]
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

let endOfSection = findLastIndex( node.children,child => child.kind=='TOKEN_COMMENT'&&child.text=='\n\n#END OF NIX_GUI SECTION')
    if(endOfSection){ //section exists
     
        node.children.splice(endOfSection,0,{kind: 'NODE_KEY_VALUE',children:[{kind:'NODE_VALUE', text:key +' = '+value+'; ⇐ADD' }]})
        node.children.splice(endOfSection+1,0,{kind: 'TOKEN_WHITESPACE',text:"\n"})
       
    }else{
        let lastIndex =findLastIndex( node.children,child => child.kind=='NODE_KEY_VALUE')
       node.children.splice(lastIndex+3,0,{kind: 'TOKEN_WHITESPACE',text:"\n\n\n\n\n\n\n\n"})
        node.children.splice(lastIndex+3,0,{kind: 'TOKEN_COMMENT',text:"\n\n#END OF NIX_GUI SECTION"})
         node.children.splice(lastIndex+3,0,{kind: 'TOKEN_WHITESPACE',text:"\n"})
        node.children.splice(lastIndex+3,0,{kind: 'NODE_KEY_VALUE',children:[{kind:'NODE_VALUE', text:key +' = '+value+'; ⇐ADD' }]})
         
        node.children.splice(lastIndex+3,0,{kind: 'TOKEN_COMMENT',text:"#START OF NIX_GUI SECTION\n\n\n"})
        node.children.splice(lastIndex+3,0,{kind: 'TOKEN_WHITESPACE',text:"\n\n\n\n\n\n\n\n"})
        
    }



}

export function addPkg(pkg,ast){
let pkgsNode = find_key_value(ast,'environment.systemPackages','node' ).findNode('self','NODE_WITH').findNode('self','NODE_LIST')
pkgsNode.children.splice(-2,0,{kind:'WHITE_SPACE',text:'\n     '})
pkgsNode.children.splice(-2,0,{kind:'NODE_IDENT',text:pkg+' ⇐ADD'})

}
export function removePkg(pkg,ast){
    let pkgsNode = find_key_value(ast,'environment.systemPackages','node' ).findNode('self','NODE_WITH').findNode('self','NODE_LIST')
   let index =  findLastIndex(pkgsNode.children,(child)=>{
       debugger
        return Ast2Text(child).replace(' ⇐ADD','')==pkg;
    })
    pkgsNode.children.splice(index-1,2)
}

export function getPkgs(ast){
let pkgsNode = find_key_value(ast,'environment.systemPackages','node' ).findNode('self','NODE_WITH').findNode('self','NODE_LIST')
let pkgs = []
    for(let child of pkgsNode.children){
        if(child.kind === 'NODE_IDENT'){
            pkgs.push(Ast2Text(child).replace(' ⇐ADD',''))
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


  export function getKeyName(key,overhead){
    let length = key.split('.').length
    let sections = length - overhead;
    for(var i=key.length-1;i>=0 ;i--){
        if(key[i]=='.'){
            
           
            if(sections<=0){
                i++
                break
            }
             sections--
            }
    }
    return key.slice(-(key.length - i))
}


export function getOverhead(pkgList){

let min = Infinity
for(let i=0;i<pkgList.length;i++){

        let len = pkgList[i].split('.').length
        if(len < min) min=len
   
    
        }
return min
}