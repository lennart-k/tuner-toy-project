<canvas bind:this={canvasElement}></canvas>

<script lang="ts">
  import { onMount } from 'svelte';

  export let canvasElement: HTMLCanvasElement;

  onMount(() => {
    // Make canvas responsive
    new ResizeObserver(() => {
      console.log("resize");
      canvasElement.width = canvasElement.clientWidth;
      canvasElement.height = canvasElement.clientHeight;
    }).observe(canvasElement);
  })
</script>
