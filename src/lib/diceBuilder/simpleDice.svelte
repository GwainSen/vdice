<script>
    import { invoke } from "@tauri-apps/api/core";


    function createDice(){
        // @ts-ignore
        var name_ts = document.getElementById("dNameArea").value;
        // @ts-ignore
        var sides_ts = document.getElementById("dSideArea").value;
        invoke('create_simple_dice',{name: name_ts, sides: sides_ts})

        //TODO:
        //clear text fields
        //send confirm
    }
</script>

<div>
    <label>
        <span class="label-text">Dice Name:</span>
        <textarea id="dNameArea" class="textarea rounded-container" rows="1" placeholder="Lucky Dice"></textarea>
    </label>

    <label>
        <span class="label-text">Side Count:</span>
        <textarea id="dSideArea" class="textarea rounded-container" rows="1" placeholder="6"></textarea>
    </label>
    <button type="button" class="btn preset-outlined-surface-300-700" onclick={createDice}>Create Dice</button>
</div>