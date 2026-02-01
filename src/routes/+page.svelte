<script lang="ts">
	import DiceMenu from "$lib/diceMenu.svelte";
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
		<DiceMenu/>
	</div>
	
	<!--Info Panel-->
	<div class ="h-20/100">
		<div id="background_top" class="card p-4 preset-filled-surface-500 h-full">Info</div>
	</div>
</div>