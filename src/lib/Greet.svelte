<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let ip = "";
  let greetMsg = "";
  let progress: number = 0;

  let to_ping = ["google.com", "junengames.me", "amazon.com"];

  async function ping() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("ping", { ip });
  }

  let best: Number;
  let best_ip: String;
  let progress_style: String;

  async function start_ping() {
    progress = 0;
    best = 99999999;
    to_ping.forEach(async (ip: string) => {
      let tmp_int: number = await invoke("ping", { ip });
      if (best > tmp_int){
        best = tmp_int;
        best_ip = ip;
      }
      progress += (1.0 / to_ping.length) * 100;
      progress_style = progress.toString() + "%";
    });
    
  }
</script>

<div>
  <div class="row">
    <!-- <input id="greet-input" placeholder="Enter something to ping..." bind:value={ip} /> -->
    <button on:click={start_ping}> Ping! </button>
  </div>
  <br>
  <div class="progress">
    <div class="progress-bar" role="progressbar" aria-valuenow="10" aria-valuemin="0" aria-valuemax="100" style:width={progress_style}></div>
  </div>
  <p bind:innerHTML={best} contenteditable="false"></p>
  <p bind:innerHTML={best_ip} contenteditable="false"></p>
</div>

<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@4.1.3/dist/css/bootstrap.min.css" integrity="sha384-MCw98/SFnGE8fJT3GXwEOngsV7Zt27NXFoaoApmYm81iuXoPkFOJwJ8ERdknLPMO" crossorigin="anonymous">

<div>
  <script src="https://code.jquery.com/jquery-3.3.1.slim.min.js" integrity="sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo" crossorigin="anonymous"></script>
<script src="https://cdn.jsdelivr.net/npm/popper.js@1.14.3/dist/umd/popper.min.js" integrity="sha384-ZMP7rVo3mIykV+2+9J3UJ46jBk0WLaUAdn689aCwoqbBJiSnjAK/l8WvCWPIPm49" crossorigin="anonymous"></script>
<script src="https://cdn.jsdelivr.net/npm/bootstrap@4.1.3/dist/js/bootstrap.min.js" integrity="sha384-ChfqqxuZUCnJSK3+MXmPNIyE6ZbWh2IMqE241rYiqJxyMiZ6OW/JmZQ5stwEULTy" crossorigin="anonymous"></script>
</div>
