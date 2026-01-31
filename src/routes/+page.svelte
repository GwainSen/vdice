<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	import DiceCard from "../lib/diceCard.svelte";
	import DiceSlot from '$lib/diceSlot.svelte';

	function roll_pressed (){
		console.log("pressed")
		invoke('custom_console_log',{msg: "Roll All"})
	}
</script>

<!--Menu-->
<!--<div class="card p-4 preset-filled-surface-500">-->
<div class="h-screen">
	<div class="space-y-4 h-10/100">
	<nav class="btn-group preset-outlined-surface-200-800 w-full h-full">
		<button  type="button" class="btn preset-tonal-secondary w-1/4">Preset</button>
		<button  type="button" class="btn preset-tonal-secondary w-1/4">Custom </button>
		<button  type="button" class="btn preset-tonal-secondary w-1/4">Preset Editor</button>
		<button  type="button" class="btn preset-tonal-secondary w-1/4">Dice Builder</button>
	</nav>
	</div>

	<!--Dice-->
	<div class="h-70/100">
		<div class="card preset-filled-surface-100 h-1/10">
			preset selector
		</div>
		<div class="w-full grid grid-row-1 grid-cols-5 h-8/10 gap-1">
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

	</div>
	<!--Info Panel-->
	<div class ="h-20/100">
		<div id="background_top" class="card p-4 preset-filled-surface-500 h-full">Info</div>
	</div>
</div>