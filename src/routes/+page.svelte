<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'

  let running = $state(true);
  let stage = $state("");
  let cloud_v = $state<null | string>(null);
  let client_v = $state<null | string>(null);
  let dlprg = $state(0);

  $effect(() => {
    // Проверить наличие обновлений через раст
    // Если найдены, перевести состояние в running

    invoke("update").then();

    listen("download-started", () => {});
    listen("download-progress", (e) => dlprg = Number.parseInt(e.payload as string));
    listen("download-finished", () => {});
    listen("stage-changed", (e) => stage = e.payload as string);
    listen("cloud-version-found", (e) => cloud_v = e.payload as string);
    listen("client-version-found", (e) => client_v = e.payload as string);
  });
</script>

<div class={`wrapper ${running && 'running'}`}>
  <div class={`img loading`}>
    <img src="./seagull.png" alt="Seagull" class="seagull">
  </div>
  <!-- <div class="worker">
    <span>{stage}</span>
  </div> -->
  <div class="mask" data-tauri-drag-region>

  </div>
</div>

<style>
  * {
    font-family: Geneva, Verdana, sans-serif;
  }

  .wrapper {
    padding: 20px;
    box-sizing: border-box;

    height: clamp(100vh, 100vh, 100vh);
    width: clamp(100vw, 100vw, 100vw);

    display: flex;
    flex-direction: column;
    justify-content: center;

    position: relative;
  }

  .wrapper > .mask {
    position: absolute;
    left: 0;
    right: 0;
    width: 100%;
    height: 100%;
  }

  .wrapper > .img {
    display: flex;
    padding: 20px;

    align-items: center;
    justify-content: center;

    overflow: hidden;
  }

  .seagull {
    object-fit: contain;
    width: 100%;
    height: 100%;
    user-select: none;
  }

  .wrapper > .loading {
    padding: 40px;
  }

  .loading > .seagull {
    animation: 2s ease 0s infinite running seaload;
  }

  .running > .img {
    padding: 60px;
  }
  .wrapper > .worker {
    height: 40px;
    
    display: flex;
    flex-direction: column;
  }

  @keyframes seaload {
    0% {
      rotate: 0deg;
    }
    30% {
      transform: scale(1.2);
    }
    60% {
      rotate: 360deg;
      transform: scale(1);
    }
    100% {
      rotate: 360deg;
    }
  }
</style>