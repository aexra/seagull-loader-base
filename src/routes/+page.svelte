<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  let running = $state(true);

  const handleStart = async () => {
    console.log("DOWNLOAD START");
  };
  const handleProgress = async (p: any) => {
    console.log(p);
  };
  const handleFinish = async () => {
    console.log("DOWNLOAD END");
  };

  $effect(() => {
    // Проверить наличие обновлений через раст
    // Если найдены, перевести состояние в running

    invoke("update").then();

    document.body.addEventListener("download-started", handleStart);
    document.body.addEventListener("download-progress", handleProgress);
    document.body.addEventListener("download-finished", handleFinish);

    return () => {
      document.body.removeEventListener("download-started", handleStart);
      document.body.removeEventListener("download-progress", handleProgress);
      document.body.removeEventListener("download-finished", handleFinish);
    };
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