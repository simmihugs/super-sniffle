<script>
  import { fade } from 'svelte/transition'

  let showFade = false
  
  function toggleFade() {
    showFade = !showFade
  }
</script>


<div class="rows">
  <h1>Animation</h1>
  <button on:click={toggleFade}>{!showFade ? "Show it!" : "Hide it!"}</button>
</div>

{#if showFade}
    <div class=" card" transition:fade={{delay: 250, duration: 300}}>
      <img
	id="asd-fav"
	src="../src-tauri/icons/sparrow-8387465_1280.jpg" />
      <p>The mean sparrow</p>
    </div>
{/if}



