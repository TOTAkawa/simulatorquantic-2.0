<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
  import init, { QuantumSimulation } from './lib/pkg/simulatorquantic.js';

  // --- ESTADOS DO EDITOR ---
  let numQubits = 3;
  let colunas = 12;
  let grade = Array(numQubits).fill(null).map(() => Array(colunas).fill(null));
  
  // --- ESTADOS DE USUÁRIO ---
  let user = "", password = "", loggedInUser = null;
  let nomeProjeto = "Novo Circuito";
  let listaProjetos = [];

  // --- ENGINE ---
  let sim, canvas, isReady = false;
  let probabilities = [];
  /** @type {any} */
  let scene, camera, renderer, sphere, arrow, controls;

  onMount(async () => {
    try {
      await init();
      sim = new QuantumSimulation(numQubits);
      initThree();
      animate();
      isReady = true;
      recalcular();
    } catch (e) { console.error("Erro no motor:", e); }
  });

  // --- LÓGICA DE DRAG & DROP ---
  function handleDragStart(e, gate) { e.dataTransfer.setData("gate", gate); }
  function handleDrop(e, q, c) {
    grade[q][c] = e.dataTransfer.getData("gate");
    grade = [...grade];
    recalcular();
  }

  // --- SINCRONIZAÇÃO COM RUST ---
  function recalcular() {
    if (!isReady || !sim) return;
    sim.reset();
    for (let c = 0; c < colunas; c++) {
      for (let q = 0; q < numQubits; q++) {
        const g = grade[q][c];
        if (!g) continue;
        if (g === 'H') sim.h(q);
        else if (g === 'X') sim.x(q);
        else if (g === 'Y') sim.y(q);
        else if (g === 'Z') sim.z(q);
        else if (g === 'CNOT' && q < numQubits - 1) sim.cnot(q, q + 1);
        else if (g === 'CCX' && q < numQubits - 2) sim.ccx(q, q + 1, q + 2);
      }
    }
    probabilities = Array.from(sim.get_probabilities());
    // Calcula vetor de Bloch do Qubit 0 para a Esfera
    const coords = Array.from(sim.get_bloch_vector(0));
    if (arrow) arrow['setDirection'](new THREE.Vector3(coords[0], coords[1], coords[2]).normalize());
  }

  // --- LOGIN SOCIAL ---
  const socialLogin = (provider) => {
    const urls = {
      google: 'https://google.com',
      github: 'https://github.com',
      apple: 'https://apple.com'
    };
    window.location.href = urls[provider];
  };

  async function salvarNoBanco() {
    if (!loggedInUser) return alert("Logue primeiro!");
    await fetch("http://localhost:3000/api/save", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ usuario: loggedInUser, nome: nomeProjeto, dados_json: JSON.stringify(grade) })
    });
    alert("Salvo no SQLite!");
  }

  // --- THREE.JS ---
  function initThree() {
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, canvas.clientWidth / canvas.clientHeight, 0.1, 1000);
    camera['position'].set(3, 3, 5);
    renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
    renderer.setSize(canvas.clientWidth, canvas.clientHeight);
    controls = new OrbitControls(camera, renderer.domElement);
    sphere = new THREE.Mesh(new THREE.SphereGeometry(2, 32, 32), new THREE.MeshBasicMaterial({ color: 0x00f7ff, wireframe: true, transparent: true, opacity: 0.1 }));
    scene.add(/** @type {any} */ (sphere));
    arrow = new THREE.ArrowHelper(new THREE.Vector3(0,1,0), new THREE.Vector3(0,0,0), 2, 0x00f7ff);
    scene.add(/** @type {any} */ (arrow));
  }

  function animate() {
    requestAnimationFrame(animate);
    if (controls) controls.update();
    renderer.render(scene, camera);
  }
</script>

<main>
  <!-- BARRA DE FERRAMENTAS -->
  <aside class="sidebar glass">
    <div class="logo">QUANT<span>SIMUL</span></div>
    
    <div class="auth-section">
      {#if !loggedInUser}
        <input bind:value={user} placeholder="Usuário" />
        <button on:click={() => loggedInUser = user}>Login Local</button>
        <div class="social-grid">
          <button on:click={() => socialLogin('google')}>G</button>
          <button on:click={() => socialLogin('github')}>Git</button>
          <button on:click={() => socialLogin('apple')}>iOS</button>
        </div>
      {:else}
        <p>Olá, <b>{loggedInUser}</b></p>
        <button class="save-btn" on:click={salvarNoBanco}>SALVAR CIRCUITO</button>
      {/if}
    </div>

    <div class="gate-box">
      <p>Arraste Portas:</p>
      <div class="gates">
        {#each ['H', 'X', 'Y', 'Z', 'CNOT', 'CCX'] as g}
          <div class="gate-item {g}" draggable="true" on:dragstart={(e) => handleDragStart(e, g)}>{g}</div>
        {/each}
      </div>
    </div>
  </aside>

  <!-- ÁREA DO EDITOR -->
  <div class="workspace">
    <div class="circuit-board glass">
      {#each grade as row, q}
        <div class="wire">
          <div class="q-label">q[{q}]</div>
          {#each row as g, c}
            <div class="slot" 
                 on:dragover|preventDefault 
                 on:drop={(e) => handleDrop(e, q, c)}
                 on:click={() => { grade[q][c] = null; recalcular(); }}>
              {#if g}
                <div class="gate-on-wire {g}">{g}</div>
                {#if g === 'CNOT'}<div class="line-v"></div>{/if}
                {#if g === 'CCX'}<div class="line-v long"></div>{/if}
              {/if}
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <div class="results-grid">
      <div class="viz glass">
        <canvas bind:this={canvas}></canvas>
      </div>
      <div class="probs glass">
        <h4>Probabilidades</h4>
        {#each probabilities as p, i}
          {#if p > 0.001}
            <div class="prob-row">
              <span class="bin">|{i.toString(2).padStart(numQubits, '0')}></span>
              <div class="track"><div class="fill" style="width: {p*100}%"></div></div>
              <span class="val">{(p*100).toFixed(0)}%</span>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</main>

<style>
  :global(body) { background: #050508; color: white; font-family: 'Inter', sans-serif; margin: 0; overflow: hidden; }
  main { display: grid; grid-template-columns: 220px 1fr; height: 100vh; padding: 10px; gap: 10px; }
  .glass { background: rgba(255,255,255,0.02); backdrop-filter: blur(15px); border: 1px solid rgba(255,255,255,0.08); border-radius: 12px; padding: 15px; }
  
  /* Sidebar */
  .logo { font-weight: 800; font-size: 1.5rem; margin-bottom: 20px; }
  .logo span { color: #00f7ff; }
  .social-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 5px; margin-top: 10px; }
  .social-grid button { background: #111; font-size: 0.7rem; padding: 5px; }
  .gates { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-top: 10px; }
  .gate-item { background: #222; border: 1px solid #444; height: 40px; display: flex; align-items: center; justify-content: center; font-weight: bold; cursor: grab; border-radius: 5px; font-size: 0.7rem; }
  
  /* Circuit */
  .workspace { display: flex; flex-direction: column; gap: 10px; }
  .circuit-board { display: flex; flex-direction: column; gap: 40px; padding: 40px; overflow-x: auto; }
  .wire { display: flex; align-items: center; border-bottom: 1px solid #222; position: relative; height: 50px; }
  .q-label { width: 40px; font-family: monospace; color: #00f7ff; }
  .slot { width: 50px; height: 50px; display: flex; align-items: center; justify-content: center; position: relative; cursor: pointer; }
  .gate-on-wire { width: 38px; height: 38px; background: #00f7ff; color: black; font-weight: bold; border-radius: 4px; z-index: 2; display: flex; align-items: center; justify-content: center; font-size: 0.8rem; }
  
  /* Fios Verticais */
  .line-v { position: absolute; width: 2px; height: 55px; background: #7000ff; top: 50%; z-index: 1; }
  .line-v.long { height: 110px; }

  /* Visualizer */
  .results-grid { display: grid; grid-template-columns: 1fr 300px; gap: 10px; flex: 1; overflow: hidden; }
  canvas { width: 100%; height: 100%; }
  .prob-row { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
  .track { flex: 1; height: 6px; background: #111; border-radius: 3px; overflow: hidden; }
  .fill { height: 100%; background: #00f7ff; box-shadow: 0 0 10px #00f7ff; transition: 0.3s; }
  .bin { font-family: monospace; font-size: 0.8rem; }
  
  button { width: 100%; background: #00f7ff; color: black; border: none; padding: 8px; border-radius: 5px; cursor: pointer; font-weight: bold; }
  input { width: 100%; background: #111; border: 1px solid #333; color: white; padding: 8px; margin-bottom: 5px; border-radius: 5px; }
</style>
