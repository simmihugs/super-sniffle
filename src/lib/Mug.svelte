<script>
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  

  const fill = {
    Dont: 0, Up: 1, Paused: 2, End: 3,
  }

  function console_log(stuff) {
    invoke('log_it', { message: stuff })
  }
  
  let time = new Date();
  $: percent = 0
  $: state = fill.Dont
  $: seconds = time.getSeconds();
  $: {
    seconds = 1
    if (state == fill.Up) {
      console_log(`${Math.floor(100 - percent)}%`)
      percent += seconds/10
      if (percent >= 90) {
	state = fill.End	
      }
    } 
  }
  
  $: suko = () => {
    if (state == fill.Up) {
      return "Stop"
    } else if (state == fill.End) {
      return "Restart"
    } else if (state == fill.Paused) {
      return "Resume"
    } else {
      return "Start"
    }
  }

  function start_stop() {
    if (state == fill.Up) {
      state = fill.Paused
    } else if (state == fill.Paused) {
      state = fill.Up
    } else if (state == fill.End) {
      percent = 0
      state = fill.Up
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
      <div class="tear"
	   style:transform="translateY({100 - seconds}%)"/>
      <div class="coffee"
	   style:transform="translateY({100 - percent}%)"/>
    </div>  
    <div class="handle"/>
    <div class="platform"/>
  </div>
  <div class="conti">  
    <button
      on:click={start_stop}>{suko()}</button>
  </div>
</div>  

