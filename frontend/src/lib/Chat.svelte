<script>
    import { sendWsEvent } from '../lib/Websocket.svelte';
    import ChatMessage from '../lib/ChatMessage.svelte';
    import ServerMessage from '../lib/ServerMessage.svelte';
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
        width: 200px;
    }

    .messages {
        flex: 1 1 auto;
        display: flex;
        flex-direction: column-reverse;
        overflow-y: auto;
    }

    .scrollable {
        width: 100%;
    }

    .chatbox {
        height: 1.3em;
    }

    .message {
        margin: 0.5em 0.2em 0.5em 0.2em;
    }
</style>


<div class=chat-wrapper>
    <div class=messages>
        <div class=scrollable>
            {#each messages as message}
                <div class=message>
                    {#if typeof message === 'string'}
                        <ServerMessage text={message} />
                    {:else}
                        <ChatMessage {...message} />
                    {/if}
                </div>
            {/each}
        </div>
    </div>
    <input class=chatbox type="text" bind:value={chat_message} on:keypress={handleKey} />
</div>
