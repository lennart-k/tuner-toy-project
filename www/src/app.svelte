<script lang="ts">
  import Tuner from "./tuner.svelte";
  import { onMount } from "svelte";
  import { EqualTemperament, FourierBuffer } from "wasmasd";

  export let tuningFreq: number;

  const fftSize = 32768;

  const buffer = new Float32Array(fftSize);
  let audioContext: AudioContext;
  let analyser: AnalyserNode;
  let samplerate: number;

  let fourierBuffer = FourierBuffer.new(fftSize);
  let temperament = EqualTemperament.new(tuningFreq);

  let currentNote: string;
  let detuningCents: number;
  let currentFrequency: number;

  let tuner: Tuner;

  onMount(async () => {
    if (!navigator.mediaDevices?.getUserMedia) {
      alert("unsupported browser");
      return;
    }
  });

  async function update() {
    analyser.getFloatTimeDomainData(buffer);
    fourierBuffer.digest_chunk(buffer);
    let maxBin = fourierBuffer.get_dominant_bin();
    let maxFreq = (maxBin * samplerate) / fftSize;
    let freqData = fourierBuffer.get_freq_data()

    if (!maxFreq) return;
    let closestResult = temperament.get_closest_note(maxFreq);
    currentNote = closestResult[0];
    detuningCents = closestResult[1];
    currentFrequency = maxFreq;

    tuner.updateSpectogram(freqData);
  }

  async function start() {
    audioContext = new AudioContext({ sampleRate: 44100 });

    samplerate = audioContext.sampleRate;
    analyser = audioContext.createAnalyser();
    analyser.fftSize = fftSize; // also chunk size for time domain data
    analyser.channelCount = 1;
    let stream = await navigator.mediaDevices.getUserMedia({
      audio: true,
    });
    let source = audioContext.createMediaStreamSource(stream);

    source.connect(analyser);

    while (true) {
      await new Promise((resolve) => requestAnimationFrame(resolve));
      await new Promise((resolve) => setTimeout(resolve, 50));
      await update();
    }
  }
</script>

<div class="app">
  <Tuner
    bind:this={tuner}
    on:click={start}
    note={currentNote}
    freq={currentFrequency}
    {detuningCents}
  />
</div>

<style>
  .app {
    width: 100%;
    height: 100%;
  }
</style>
