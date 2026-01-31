<script lang="ts">
	import DiceCard from "../lib/diceCard.svelte";


	function roll_pressed (){
		console.log("pressed")
	}
</script>

<!--Menu-->
<!--<div class="card p-4 preset-filled-surface-500">-->
<div class="h-screen">
	<div class="space-y-4 h-10/100">
	<nav class="btn-group preset-outlined-surface-200-800 w-full h-full">
		<button  type="button" class="btn preset-tonal-secondary w-1/3"> Dice</button>
		<button  type="button" class="btn preset-tonal-secondary w-1/3"> Dice</button>
		<button  type="button" class="btn preset-tonal-secondary w-1/3"> Dice</button>
	</nav>
	</div>

	<!--Dice-->
	<div class="h-70/100">
		<div class="w-full grid grid-cols-5 h-9/10">
			<DiceCard name="D1"></DiceCard>
			<DiceCard name="D2"></DiceCard>
			<DiceCard name="D3"></DiceCard>
			<DiceCard name="D4"></DiceCard>
			<DiceCard name="D5"></DiceCard>
		</div>
		<div class="flex flex-col items-center justify-cente h-1/10">
			<button class="btn preset-filled-tertiary-500 h-full content-center" onclick={roll_pressed}>
				Roll
			</button>
		</div>

	</div>
	<!--Info Panel-->
	<div class ="h-20/100">
		<div id="background_top" class="card p-4 preset-filled-surface-500 h-full">Info</div>
	</div>
</div>