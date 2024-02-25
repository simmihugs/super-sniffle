<script>
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  

  const fill = {
    Dont: 0, Up: 1, Paused: 2,
  }
  
  let time = new Date();
  $: percent = 0
  $: state = fill.Dont
  $: seconds = time.getSeconds();
  $: {
    seconds = 1
    if (state == fill.Up) {
      percent += seconds/10
      if (percent >= 100) {
	state = fill.Dont
      }
    }
  }
  
  function console_log(stuff) {
    invoke('log_it', { message: stuff })
  }
  
  function start_stop() {
    if (state == fill.Up) {
      state = fill.Paused
    } else if (state == fill.Paused) {
      state = fill.Dont
      percent = 0
    } else {
      state = fill.Up
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

<div class="rows">
  <div>
    <div class="mug">
      <div class="coffee"
	   style:transform="translateY({100 - percent}%)"/>
    </div>  
    <div class="handle"/>
    <div class="platform"/>
  </div>
  <div class="conti">  
    <button
      on:click={start_stop}>{ state == fill.Up ? "Stop" : state ==
      fill.Dont ? "Start" : "Reset" }</button>
  </div>
</div>  

