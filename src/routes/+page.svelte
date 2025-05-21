<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'

  let running = $state(true);

  $effect(() => {
    // Проверить наличие обновлений через раст
    // Если найдены, перевести состояние в running

    invoke("update").then();

    listen("download-started", () => console.log("START"));
    listen("download-progress", (e) => console.log(e.payload));
    listen("download-finished", () => console.log("FIN"));
    listen("stage-changed", (e) => console.log(e.payload));
    listen("cloud-version-found", (e) => console.log(e.payload));
    listen("client-version-found", (e) => console.log(e.payload));
  });
</script>

<div class={`wrapper ${running && 'running'}`}>
  <div class={`img loading`}>
    <img src="./seagull.png" alt="Seagull" class="seagull">
  </div>
  <div class="worker">
    
  </div>
</div>

<style>
  .wrapper {
    padding: 20px;
    box-sizing: border-box;

    height: clamp(100vh, 100vh, 100vh);
    width: clamp(100vw, 100vw, 100vw);

    display: flex;
    flex-direction: column;
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
  }

  @keyframes seaload {
    0% {
      rotate: 0deg;
    }
    60% {
      rotate: 360deg;
    }
    100% {
      rotate: 360deg;
    }
  }
</style>