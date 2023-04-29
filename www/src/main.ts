import App from './app.svelte'

globalThis.app = new App({
    target: document.body,
    props: {
        tuningFreq: 442
    }
})
