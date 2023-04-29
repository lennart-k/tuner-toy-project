<script lang="ts">
  import { onMount } from "svelte";
  import ResponsiveCanvas from "./responsive-canvas.svelte";

  let canvasElement: HTMLCanvasElement;
  let canvasContext: CanvasRenderingContext2D;

  onMount(() => {
    canvasContext = canvasElement.getContext("2d");
  });

  export function update(freqData: Float32Array) {
    const fftSize = freqData.length;

    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
    let [boxWidth, boxHeight] = [
      canvasElement.width / fftSize,
      (canvasElement.height / fftSize) * 200,
    ];
    canvasContext.globalAlpha = 0.9;
    canvasContext.fillStyle = `hsl(173, 80%, 20%)`;
    for (const i in freqData) {
      let pos =
        (canvasElement.width * Math.log2(Number(i))) /
        Math.log2(freqData.length);
      let pillHeight = (boxHeight * freqData[i]) / 4;
      canvasContext.fillRect(
        pos - 4,
        canvasElement.height - pillHeight,
        boxWidth + 8,
        pillHeight
      );
    }
  }
</script>

<ResponsiveCanvas bind:canvasElement />
