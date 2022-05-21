<div class="container">
    <svg width="31" height="28" viewBox="0 0 31 28" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M21.7884 11.3357C21.7884 17.1636 17.0683 21.8881 11.2456 21.8881C5.42302 21.8881 0.70285 17.1636 0.70285 11.3357C0.70285 5.50786 5.42302 0.783417 11.2456 0.783417C17.0683 0.783417 21.7884 5.50786 21.7884 11.3357ZM3.55991 11.3357C3.55991 15.5843 7.00093 19.0284 11.2456 19.0284C15.4904 19.0284 18.9314 15.5843 18.9314 11.3357C18.9314 7.0872 15.4904 3.64306 11.2456 3.64306C7.00093 3.64306 3.55991 7.0872 3.55991 11.3357Z" fill="#BEB3B3"/>
        <rect width="13.5518" height="3.21546" transform="matrix(0.798128 0.602488 -0.601794 0.798652 19.5064 16.4008)" fill="#BEB3B3"/>
        </svg>
        
    <input type="text" bind:value={inputValue} on:keyup={()=>keyUpFn && keyUpFn()}>
</div>

<script lang='ts'>
import { OptionInputValue } from "@src/store/store";


export let inputValue:string = '';
export let keyUpFn:Function;

$:inputValue = $OptionInputValue;
</script>

<style>
    div.container{
    position: relative;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: left;
    background-color: #717171;
    width: 700px;
    margin: auto;
    border-radius: 13px;
    padding:10px;
    margin-top:10px
    }
    input{
    height: 40px;
    background-color: transparent;
    border: none;
    font-size: 30px;
    color: white;
    width: 100%;
    margin-left: 20px;
    margin-right: 50px;
    text-align: center;
    outline: none;
    }
</style>