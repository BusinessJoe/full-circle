<script>
    import { sendWsEvent } from '../lib/Websocket.svelte';
    export let websocket;
    export let messages;
    let chat_message = "";

    function handleKey(event) {
        if (event.key === "Enter" && chat_message) {
            sendWsEvent(websocket, "ChatMessage", chat_message);
            chat_message = "";
        }
    }

    $: messages, console.log(messages);
</script>


<style>
    .chat-wrapper {
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        height: 100%;
        background-color: red;
    }

    .messages {
        flex: 1 1 auto;
        display: flex;
        flex-direction: column-reverse;
        overflow-y: scroll;
    }

    .scrollable {
        width: 100%;
        background-color: green;
    }

    .chatbox {
        height: 1.3em;
    }
</style>


<div class=chat-wrapper>
    <div class=messages>
        <div class=scrollable>
            {#each messages as message}
                <div>
                    {message}
                </div>
            {/each}
        </div>
    </div>
    <input class=chatbox type="text" bind:value={chat_message} on:keypress={handleKey} />
</div>
