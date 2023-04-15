<script lang="ts">
  import Tuner from "./tuner.svelte";
  import { onMount } from "svelte";
  import { TestAnalyser, EqualTemperament } from "wasmasd";

  export let tuningFreq: number;

  const fftSize = 32768;
  // const fftSize = 4096
  const buffer = new Uint8Array(fftSize);
  let audioContext: AudioContext|OfflineAudioContext;
  let analyser: AnalyserNode;
  let samplerate: number;
  let testAnalyser: TestAnalyser;

  let temperament = EqualTemperament.new(tuningFreq);

  let running = false;
  let canvasElement: HTMLCanvasElement, canvasContext: CanvasRenderingContext2D;

  let currentNote: string;
  let detuningCents: number;
  let currentFrequency: number;

  onMount(async () => {
    if (!navigator.mediaDevices?.getUserMedia) {
      alert("unsupported browser");
      return;
    }

    // Make canvas responsive
    new ResizeObserver(() => {
      console.log('resize')
      canvasElement.width = canvasElement.clientWidth
      canvasElement.height = canvasElement.clientHeight  
    }).observe(canvasElement)
    canvasContext = canvasElement.getContext("2d");
  });

  async function update() {
    analyser.getByteFrequencyData(buffer);

    let maxFreq = testAnalyser.get_dominant_frequency(buffer);
    // console.log(maxFreq);
    if (!maxFreq) return
    let closestResult = temperament.get_closest_note(maxFreq);
    currentNote = closestResult[0];
    detuningCents = closestResult[1];
    currentFrequency = maxFreq;

    canvasContext.clearRect(
      0,
      0,
      canvasElement.width,
      canvasElement.height
    );
    let [boxWidth, boxHeight] = [
      canvasElement.width / fftSize,
      (canvasElement.height / fftSize) * 20,
    ];
    canvasContext.globalAlpha = 0.9;
    canvasContext.fillStyle = `hsl(173, 80%, 20%)`;
    for (const i in buffer) {
      let pos = canvasElement.width*Math.log2(Number(i))/Math.log2(buffer.length)
      let pillHeight = boxHeight * buffer[i] * 3;
      canvasContext.fillRect(
        pos-4,
        canvasElement.height - pillHeight,
        boxWidth + 8,
        pillHeight
      );
    }
  }

  async function start() {
    if (running) return;
    running = true;
    audioContext = new AudioContext({ sampleRate: 44100 });
    // audioContext.destination.disconnect()

    samplerate = audioContext.sampleRate;
    console.log(samplerate)
    analyser = audioContext.createAnalyser();
    analyser.fftSize = fftSize;
    analyser.channelCount = 1;
    analyser.smoothingTimeConstant = 0;
    testAnalyser = TestAnalyser.new(tuningFreq, fftSize, samplerate);
    let stream = await navigator.mediaDevices.getUserMedia({
      audio: true,
    });
    let source = audioContext.createMediaStreamSource(stream);
    source.connect(analyser);
    while (true) {
      await new Promise((resolve) => requestAnimationFrame(resolve));
      await update();
    }
  }
</script>

<div class="app">
  <Tuner
    on:click={start}
    note={currentNote}
    freq={currentFrequency}
    {detuningCents}
    bind:canvasElement
  />
</div>

<style>
  .app {
    width: 100%;
    height: 100%;
  }
</style>
