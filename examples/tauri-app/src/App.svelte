<script>
    import Greet from './lib/Greet.svelte'
    import {execute} from 'tauri-plugin-clubgermes-firebase-api'

    import {sendNotification} from "./helpers/notifications-test";

    import {getMyToken} from 'tauri-plugin-clubgermes-firebase-api'

    let myFCMToken = ''

    function updateResponseToken(returnTokenValue) {
        myFCMToken += `[${new Date().toLocaleTimeString()}]` + (typeof returnTokenValue === 'string' ? returnTokenValue : JSON.stringify(returnTokenValue)) + '<br>'
    }

    function _executeToken() {
        getMyToken().then(updateResponseToken).catch(updateResponseToken)
    }


    let response = ''

    function updateResponse(returnValue) {
        response += `[${new Date().toLocaleTimeString()}]` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
    }

    function _execute() {
        execute().then(updateResponse).catch(updateResponse)
    }
</script>

<main class="container">
    <h1>Welcome to Tauri!</h1>

    <div class="row">
        <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite Logo"/>
        </a>
        <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo"/>
        </a>
        <a href="https://svelte.dev" target="_blank">
            <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo"/>
        </a>
    </div>

    <p>
        Click on the Tauri, Vite, and Svelte logos to learn more.
    </p>

    <div class="row">
        <Greet/>
    </div>

    <div>
        <button on:click="{_execute}">Execute</button>
        <div>{@html response}</div>
    </div>

    <p>

    </p>

    <div>
        <button class="btn" id="notification" on:click={sendNotification}>
            Send test notification
        </button>
    </div>

    <p>

    </p>

    <div>
        <button on:click="{_executeToken}">Get FCM token</button>
        <div>{@html myFCMToken}</div>
    </div>

</main>

<style>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }

    .logo.svelte:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
    }
</style>
