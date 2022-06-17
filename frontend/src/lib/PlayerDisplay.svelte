<script>
    export let players;
</script>

<div>
    Players:
    {#each players as player}
        <div class:player>
            <div>
                Name: {player.name}
            </div>
            <div>
                Host: {player.is_host}
            </div>
        </div>
    {/each}
</div>

<style>
    .player {
        background-color: blue;
    }
</style>
