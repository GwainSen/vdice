<script lang="ts">
	import DiceMenu from "$lib/diceMenu/diceMenu.svelte";
	import DiceBuilder from "$lib/diceBuilder/diceBuilder.svelte";

	var current_menu = $state("Preset")

	function changeMenu(newMenu: string){
		current_menu = newMenu;
		console.log(current_menu)
	}
</script>

<!--Menu-->
<!--<div class="card p-4 preset-filled-surface-500">-->
<div class="h-screen">
	<div class="space-y-4 h-10/100">
		<nav class="btn-group preset-outlined-surface-200-800 w-full h-full">
			<button type="button" class="btn preset-tonal-secondary w-1/4" onclick={()=>changeMenu("Preset")}>Preset</button>
			<button type="button" class="btn preset-tonal-secondary w-1/4" onclick={()=>changeMenu("Custom")}>Custom</button>
			<button type="button" class="btn preset-tonal-secondary w-1/4" onclick={()=>changeMenu("Preset_Editor")}>Preset Editor</button>
			<button type="button" class="btn preset-tonal-secondary w-1/4" onclick={()=>changeMenu("Dice_Builder")}>Dice Builder</button>
		</nav>
	</div>

	<!--Dice-->
	<div class="h-70/100">
		{#if current_menu==="Preset"}
				<DiceMenu/>
		{:else if current_menu==="Custom"}
			<div>Custom dice menu</div>
		{:else if current_menu==="Preset_Editor"}
			<div>preset editor</div>
		{:else if current_menu==="Dice_Builder"}
			<DiceBuilder/>
		{/if}
	</div>

	<!--Info Panel-->
	<div class ="h-20/100">
		<div id="background_top" class="card p-4 preset-filled-surface-500 h-full">Info</div>
	</div>
</div>