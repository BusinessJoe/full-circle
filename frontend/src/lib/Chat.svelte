<script>
    import { onMount, onDestroy } from 'svelte';
    import ChatMessage from '../lib/ChatMessage.svelte';
    import ServerMessage from '../lib/ServerMessage.svelte';
    export let websocket;
    let messages = [];
    let chat_message = "";

    function handleKey(event) {
        if (event.key === "Enter" && chat_message) {
            websocket.send("ChatMessage", chat_message);
            chat_message = "";
        }
    }

    onMount(() => {
        console.log("Adding handlers");
        websocket.addEventListener("ChatMessage", (payload) => {
            // We can't just use messages.push(), since the mutation will not trigger an update on its own.
            messages = [...messages, payload];
        });
        websocket.addEventListener("SecretChatMessage", (payload) => {
            payload.secret = true;
            messages = [...messages, payload];
        });
        websocket.addEventListener("ServerMessage", (payload) => {
            console.log(payload);
            messages = [...messages, payload];
        });
    });
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
    <input class=chatbox type="text" bind:value={chat_message} on:keypress={handleKey} maxlength="50" />
</div>
