

    <svg width="41" height="39" viewBox="0 0 41 39" fill="none" xmlns="http://www.w3.org/2000/svg"  on:click={mark}>
        <rect x="1" y="1" width="38.4348" height="36.2667" rx="3" stroke="#515151" stroke-width="2"/>
        <path class:marked={marked} d="M37.7142 10.419C37.7142 10.0836 33.8484 5.51342 33.8484 5.51342L31.6959 5.51344L22.424 16.2683L16.3311 23.8958C16.3311 23.8958 16.1627 23.5254 13.116 20.4242C10.0693 17.323 8.87084 16.2683 8.87084 16.2683C8.87084 16.2683 8.53975 16.7377 7.08268 18.6499C5.6256 20.562 4.4016 21.3129 4.56717 21.6483C4.73274 21.9838 5.43658 21.38 10.7679 27.2838C12.8277 29.5648 15.3377 32.8857 16.0662 33.6572H17.3491C17.5176 33.6572 17.7689 33.0636 17.8133 33.0186C17.9129 32.9179 20.1314 28.5776 27.1846 20.4242C34.2379 12.2708 37.7142 10.7545 37.7142 10.419Z" fill="#DBDDDB"/>
        </svg>
        

    <script>
import  { ast,installedPkgs, needsSaving } from "@src/store/store";
import { addPkg, getPkgs, removePkg } from "@src/utils/globalFunctions";

export let name;
       
     
          
        let marked = $installedPkgs.includes(name)
        
        function mark(){
            marked=!marked
            if(marked){
                addPkg(name,$ast)
                $installedPkgs=getPkgs($ast)
                console.log($installedPkgs)
            }else{
                removePkg(name,$ast)
                $installedPkgs=getPkgs($ast)
                console.log($installedPkgs)
            }
            $needsSaving=true;
        }

    </script>


<style>
    .marked{
        fill:#0cee0c
    }
    svg{
         top:2px;
         left: 4px;
        
    }
    svg:active{
        transform: scale(0.8);
    }
</style>