<script>
  import Circle from './lib/Circle.svelte'
  import Table from './lib/Table.svelte'

  import { invoke } from '@tauri-apps/api/tauri'

  let my_input = ''
  function console_log(stuff) {
    invoke('log_it', {message: stuff})
  }

  function spill_it() {
    console_log(my_input)
  }

  function echo_it() {
    invoke('echo_it', {message: 'it'})
  }

  let greetInputEl = ''
  let greetMsgEl = ''
  async function greet() {
    greetMsgEl = await invoke("greet", { name: greetInputEl})
  }
</script>

<main class="container">
  <h1>Example tauri app</h1>

  <div class='row'>
    <button on:click={spill_it}>log it</button>    
    <input id="greet-input" placeholder="Enter a name..."
	   bind:value="{my_input}" />
  </div>

  <div class='row'>
    <button on:click={echo_it}>echo it</button>    
  </div>

  <div class='row'>
    <button on:click={greet}>cool</button>    
    <input id="greet-input" placeholder="Enter a name..."
	   bind:value="{greetInputEl}" />
    <p>{greetMsgEl}</p>
  </div>
  
  <Circle />
  <Table />
</main>

