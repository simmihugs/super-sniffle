<script>
  import { invoke } from '@tauri-apps/api/tauri'
  
  let fill = false;
  let percent = 0
  
  function console_log(stuff) {
    invoke('log_it', { message: stuff })
  }
  
  function start_stop() {
    fill = !fill
    console_log(fill ? "true" : "false")
  }

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
      on:click={start_stop}>click me</button>
  </div>
</div>  

