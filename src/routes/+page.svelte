<!--
  +page.svelte — la pagina principale di Plokk
  Per ora è solo una base pulita: titolo, bottone, risultato.
  Nei prossimi sprint aggiungeremo la lista dei file, il
  pannello glass, il drag & drop.
-->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // $state: variabile reattiva. Quando cambia, la UI si aggiorna da sola.
  let response = $state("");

  // Chiama il comando Rust "ping" e mette il risultato in `response`.
  async function testConnection() {
    response = await invoke("ping");
  }
</script>

<main class="container">
  <h1>Plokk</h1>
  <p class="subtitle">Sprint 1 — test connessione Rust ↔ Svelte</p>

  <button onclick={testConnection}>Test connessione</button>

  {#if response}
    <p class="result">{response}</p>
  {/if}
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    background: #f5f5f7;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui,
      sans-serif;
  }

  .container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
  }

  h1 {
    margin: 0;
    font-size: 2.5rem;
    font-weight: 700;
    letter-spacing: -0.03em;
    color: #0d0d0d;
  }

  .subtitle {
    margin: 0;
    font-size: 0.9rem;
    color: #6b7280;
  }

  button {
    margin-top: 1rem;
    padding: 0.7rem 1.4rem;
    border-radius: 10px;
    border: 1px solid #d1d5db;
    background: white;
    color: #0d0d0d;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
  }

  button:hover {
    border-color: #4d6bfe;
    color: #4d6bfe;
    transform: translateY(-1px);
  }

  button:active {
    transform: scale(0.97);
  }

  .result {
    margin-top: 1rem;
    font-size: 1.8rem;
    font-weight: 700;
    color: #4d6bfe;
    animation: pop 0.3s cubic-bezier(0.34, 1.4, 0.4, 1);
  }

  @keyframes pop {
    from {
      opacity: 0;
      transform: scale(0.8);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
</style>
