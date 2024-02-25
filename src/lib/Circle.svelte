<script>
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from 'svelte';

  let up = true
  let percent = 100
  let time = new Date();
  $: seconds = time.getSeconds();
  $: active = false
  $: {
    seconds = 1
    if (active) {
      if (up) {
	percent -= seconds/10
	if (percent <= 0) {
	  up = false
	}
      } else {
	percent += seconds/10
	if (percent >= 100) {
	  up = true
	}
      }
      
    }
  }

  async function update() {
    if (active) {
      active = false
    } else {
      active = true
    }
  }

  onMount(() => {
    const interval = setInterval(() => {
      time = new Date();
    }, 50);
    
    return () => {
      clearInterval(interval);
    };
  });
</script>

<div class="row">
  <div class="container">
    <div class="above">
      <div class="circle">
	<div
	  id="filling"
	  class="filling"
	  style:transform='translateY({percent}%)'
	  style:background='{up ? "tomato" : "#34704a"}'>
	</div>
      </div>
      <p class="percent">
	{Math.floor(percent)}%
      </p>
    </div>
  </div>
</div>

<div class="row">
  <button on:click={update}>{active ? "Stop" : "Start"}</button>
</div>
