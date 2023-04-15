<div class="tuner" on:click>
    <p class="note">{note}</p>
    <h2>{freq}</h2>
    <p>{Math.round(detuningCents)}</p>
    <DetuningSlider {detuningCents}></DetuningSlider>
    <canvas bind:this={canvasElement}></canvas>
</div>

<script lang="ts">
    import DetuningSlider from "./detuningSlider.svelte";

    export let note: string;
    export let freq: number
    export let detuningCents: number
    
    export let canvasElement: HTMLCanvasElement;
</script>

<style>
    .tuner {
        width: 100%;
        height: 100%;
        background: linear-gradient(hsl(173, 14%, 20%), hsl(210, 15%, 15%));
        color: white;
        text-align: center;
        padding: 12px;
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .note {
        font-size: 3em;
        font-weight: bolder;
    }

    canvas {
        width: 100%;
        height: 70%;
        position: absolute;
        bottom: 0;
        left: 0;
    }
</style>