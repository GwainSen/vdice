<script lang="ts">

    var current_menu = $state("Simple_Dice")

    function changeMenu(newMenu: string){
        if(newMenu != current_menu){
            current_menu = newMenu;
        }
    }
</script>


<div class="card h-full w-full">
    <div class="space-y-4 h-10/100">
		<nav class="btn-group preset-outlined-surface-200-800 w-full h-full">
			<button type="button" class="btn preset-tonal-primary w-1/3" onclick={()=>changeMenu("Simple Dice")}>Simple Dice</button>
			<button type="button" class="btn preset-tonal-primary w-1/3" onclick={()=>changeMenu("WIP")}>WIP</button>
			<button type="button" class="btn preset-tonal-primary w-1/3" onclick={()=>changeMenu("WIP")}>WIP</button>
		</nav>
	</div>
    {#if current_menu ==="Simple_Dice"}
        <div>Simple Dice</div>
    {:else if current_menu === "WIP"}
        <div>WIP</div>
    {/if}
</div>