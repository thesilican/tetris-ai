import "sanitize.css";
import "sanitize.css/typography.css";
import { HumanPlayer } from "./player/human";
import { Canvas } from "./render/canvas";
import { GameRenderer } from "./render/game-renderer";
import "./styles.css";
import WasmWorker from "./wasm/worker?worker";
import { AiPlayer } from "./player/ai";

const startGameButton = document.getElementById(
  "start-game"
) as HTMLButtonElement;
const startSelect = document.getElementById(
  "start-select"
) as HTMLSelectElement;
const description = document.getElementById(
  "description"
) as HTMLParagraphElement;
const aiSpeed = document.getElementById("ai-speed") as HTMLInputElement;
const aiControls = document.getElementById("ai-controls") as HTMLDivElement;
const aiOutput = document.getElementById("ai-output") as HTMLPreElement;
const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
const canvas = new Canvas(canvasElement);
const worker = new WasmWorker();
const renderer = new GameRenderer(canvas);
renderer.adjustScaling();
renderer.renderStartScreen();
let player: HumanPlayer | AiPlayer | undefined;

function updateControls(value: string) {
  if (value === "human") {
    description.innerText = "Play a game of tetris (see controls).";
    aiControls.style.display = "none";
  } else if (value.startsWith("bot-")) {
    aiControls.style.display = "";
    const bot = value.slice(4);
    if (bot === "simple-ai") {
      description.innerText =
        "A simple AI that looks 1 move deep and agressively minimizes board height.";
    }
  }
}
// TODO: remove
startSelect.value = "human";
updateControls("human");

startSelect.addEventListener("change", (e) => {
  const value = (e.target as HTMLSelectElement).value;
  updateControls(value);
});

startGameButton.addEventListener("click", () => {
  if (player !== undefined) {
    player.stop();
  }
  if (startSelect.value === "human") {
    player = new HumanPlayer(renderer);
    aiOutput.innerText = "";
  } else if (startSelect.value.startsWith("bot-")) {
    player = new AiPlayer(
      startSelect.value.slice(4),
      renderer,
      worker,
      aiOutput
    );
  } else {
    throw new Error("unreachable");
  }
  player.start();
});

aiSpeed.addEventListener("change", (e) => {
  const speed = parseInt((e.target as HTMLInputElement).value, 10);
  if (player instanceof AiPlayer) {
    player.setSpeed(speed);
  }
});
