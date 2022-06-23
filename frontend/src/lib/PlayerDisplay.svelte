<script>
    export let players;

</script>

<div>
    Players:
    {#each players as player}
        <div class={player.has_answer ? 'player-answered' : 'player'}>
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
    }
    .player-answered {
        background-color: green;
    }
</style>
