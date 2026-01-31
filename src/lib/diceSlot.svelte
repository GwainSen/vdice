<script>
    import { invoke } from '@tauri-apps/api/core';

    import DiceCard from "./diceCard.svelte";

    let { nameS = 'world' } = $props();

    function cardButtonPressed(){
        invoke('custom_console_log',{msg: nameS})
    }
</script>

<div class="card preset-outlined-primary-500 h-full">
    <div class="card preset-outlined-secondary-500 h-9/10">
    <DiceCard name={nameS}></DiceCard>
    </div>
    <div class="flex flex-col items-center justify-cente h-1/10">
    <button onclick={cardButtonPressed} class="btn preset-tonal-tertiary">Roll</button>
    </div>
</div>

