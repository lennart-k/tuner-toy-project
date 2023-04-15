<div class="detuning-slider">
    <div class="slider-track"></div>
    <div class="slider-thumb-wrapper" style="transform: translateX({detuningCents}%)">
        <div class="slider-thumb"></div>
    </div>
</div>
<script lang="ts">
    export let detuningCents: number
</script>
<style>
    .detuning-slider {
        --slider-height: 40px;

        width: clamp(200px, 80%, 400px);
        width: clamp(200px, 80%, 400px);
        height: var(--slider-height);
        position: relative;
        contain: strict;
    }

    .slider-track {
        position: absolute;
        width: 100%;
        height: 100%;
        background: rgba(255 255 255 / .4);
        border-radius: calc(var(--slider-height) / 2);
    }

    .slider-thumb-wrapper {
        position: absolute;
        left: calc(var(--slider-height) / 2);
        top: 0;
        width: calc(100% - var(--slider-height));
        height: 100%;
        transition: transform .2s ease-out;
    }

    .slider-thumb {
        background: rgba(255 255 255 / 1);
        position: absolute;
        height: 100%;
        aspect-ratio: 1;
        border-radius: calc(var(--slider-height) / 2);
        left: calc(50% - var(--slider-height) / 2);
        top: 0;
    }
</style>