const { invoke } = window.__TAURI__.tauri;

async function greet() {
  await invoke("greet", { name: greetInputEl.value });
}

async function setPlayerColor() {
  let color = document.forms["playerColor"]["color"].value === "true";
  invoke("set_player_color", {white: color});
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#playerColor").addEventListener("submit", (e) => {
    e.preventDefault();
    setPlayerColor();
  });
});
