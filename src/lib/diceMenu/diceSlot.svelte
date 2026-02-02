<script>
    import { invoke } from '@tauri-apps/api/core';

    import DiceCard from './diceCard.svelte';

    let { nameS = 'world' } = $props();

    var last_roll = $state("20")

    function roll_dice_x(){
        invoke('my_custom_command', {dice: nameS}).then((helper) => last_roll = helper);
    }

    function cardButtonPressed(){
        invoke('custom_console_log',{msg: nameS});
        roll_dice_x()
    }
</script>

<div class="card preset-outlined-primary-500 h-full">
    <div class="card h-9/10 items-center">
        <div class="flex flex-col items-center h-full">
            <div class="h-3/100"></div> <!--spacer-->
            <div class="card preset-tonal-primary min-w-5/10"> <!--dice name-->
                <div class="flex flex-col items-center">
                    {nameS}
                </div>
            </div> 
            <div class="h-5/100"></div> <!--spacer-->
            <div class="h-65/100 w-7/10"><!--dice icon-->
                <div class="card preset-tonal-tertiary h-full w-full"></div>
            </div>
            <div class="h-5/100"></div> <!--spacer-->
            <div class="h-9/100 w-30/100"> <!--dice roll-->
                <div class="card preset-tonal-tertiary h-full w-full">
                    <div class="flex flex-col items-center h-full">
                        {#key last_roll}
                            {last_roll}
                        {/key}
                    </div>
                </div>
            </div>     

        </div>
    </div>
    <div class="flex flex-col items-center justify-cente h-1/10">
        <button onclick={cardButtonPressed} class="btn preset-tonal-tertiary h-full w-full">Roll</button>
    </div>
</div>

