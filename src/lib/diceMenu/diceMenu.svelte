<script>
    import { invoke } from '@tauri-apps/api/core';

	import DiceSlot from '$lib/diceMenu/diceSlot.svelte';

    function roll_pressed (){
		invoke('custom_console_log',{msg: "Roll All"})
	}
</script>

<div class="card preset-filled-surface-100 h-1/10">
    preset selector
</div>
<div class="w-full grid grid-row-1 grid-cols-5 h-8/10 gap-1"> <!--change later to allow diffrent counts of dice-->
    <DiceSlot nameS="D1"></DiceSlot>
    <DiceSlot nameS="D2"></DiceSlot>
    <DiceSlot nameS="D3"></DiceSlot>
    <DiceSlot nameS="D4"></DiceSlot>
    <DiceSlot nameS="D5"></DiceSlot>
</div>
<div class="flex flex-col items-center justify-cente h-1/10">
    <button class="btn preset-filled-tertiary-500 h-full content-center" onclick={roll_pressed}>
        Roll All
    </button>
</div>