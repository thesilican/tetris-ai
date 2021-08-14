import { Frame, printFrame, recognizeFrame } from "./frame";
import { deepEqual } from "./util";

// Window Typedef
declare global {
  interface Window {
    PIXI: any;
    getApp: any;
  }
}

function getFrameNum() {
  const span = document.querySelector(
    "#replaytools_timestamp span"
  ) as HTMLSpanElement;
  const text = span!.innerText;
  const num = parseInt(text.match(/frame (\d+)/)![1], 10);
  return num;
}

function getFrameName() {
  const spans = document.querySelectorAll<HTMLSpanElement>("#data_replay span");
  const p1 = spans.item(0).innerText.toLowerCase();
  const p2 = spans.item(1).innerText.toLowerCase();
  const round = spans.item(2).innerText;
  return `${p1}-${p2}-${round}`;
}

function captureCanvasFrame(app: any) {
  // Hide replay stuff
  const p1 = app.stage.children[2].children[1].children[0];
  const p2 = app.stage.children[2].children[1].children[1];
  p1.children[18].visible = false;
  p1.children[19].visible = false;
  p2.children[18].visible = false;
  p2.children[19].visible = false;

  // Capture active block
  p1.children[6].visible = false;
  p2.children[6].visible = false;
  p1.children[15].visible = true;
  p2.children[15].visible = true;
  const active = app.renderer.extract.canvas(app.stage) as HTMLCanvasElement;

  // Capture board
  p1.children[6].visible = true;
  p2.children[6].visible = true;
  p1.children[15].visible = false;
  p2.children[15].visible = false;
  const board = app.renderer.extract.canvas(app.stage) as HTMLCanvasElement;

  // Reset visibility
  p1.children[6].visible = true;
  p2.children[6].visible = true;
  p1.children[15].visible = true;
  p2.children[15].visible = true;
  return { active, board };
}

async function captureFrames(app: any) {
  // Pause, if not already paused
  const playPauseButton = document.querySelector<HTMLButtonElement>(
    "#replaytools_button_playpause"
  );
  if (playPauseButton?.innerText !== "ǣ") {
    playPauseButton?.click();
  }
  // Press back button until frame is 0
  while (getFrameNum() !== 0) {
    document
      .querySelector<HTMLButtonElement>("#replaytools_button_backward_large")
      ?.click();
    await new Promise((res) => requestAnimationFrame(res));
  }

  const frames: Frame[] = [];
  for (let i = 0; true; i++) {
    // Capture frame
    const canvasFrame = captureCanvasFrame(app);
    if (getFrameNum() !== i) {
      break;
    }
    const frame = recognizeFrame(canvasFrame);
    printFrame(frame);
    frames.push(frame);

    // Advance frame
    document
      .querySelector<HTMLButtonElement>("#replaytools_button_forward_frame")
      ?.click();
    await new Promise((res) => requestAnimationFrame(res));
  }
  // Deduplicate Frames
  const framesDeduped = [];
  let prevFrame: Frame | null = null;
  for (const frame of frames) {
    if (deepEqual(frame, prevFrame)) {
      continue;
    }
    prevFrame = frame;
    framesDeduped.push(frame);
  }
  console.log(framesDeduped);
  return framesDeduped;
}

function downloadJSON(name: string, obj: any) {
  const url =
    "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(obj));
  const a = document.createElement("a");
  a.download = name;
  a.href = url;
  document.body.appendChild(a);
  a.click();
  a.remove();
}

function main(app: any) {
  const container = document.createElement("div");
  container.style.position = "fixed";
  container.style.top = "0";
  container.style.left = "0";
  container.style.display = "flex";
  container.style.flexDirection = "column";
  container.style.zIndex = "9999999";
  document.body.appendChild(container);

  const startButton = document.createElement("button");
  startButton.style.padding = "10px";
  startButton.innerText = "Record frames";
  container.appendChild(startButton);

  startButton.addEventListener("click", async () => {
    startButton.innerText = "Recording...";
    const frames = await captureFrames(app);
    downloadJSON(`${getFrameName()}.json`, frames);
    startButton.innerText = "Record frames";
  });
}

function bootstrap() {
  let int = setInterval(() => {
    if (window.PIXI) {
      clearInterval(int);
    } else {
      return;
    }
    // Trap PIXI.Application
    window.PIXI.Application = new Proxy(window.PIXI.Application, {
      construct(target, args) {
        const app = new target(...args);
        main(app);
        window.getApp = () => app;
        return app;
      },
    });
  });
  let int2 = setInterval(async () => {
    const preload = document.querySelector("#preload.ns.ready");
    const button = document.querySelector<HTMLButtonElement>("#return_button");
    if (button && preload) {
      clearInterval(int2);
    } else {
      return;
    }
    await new Promise((res) => setTimeout(res));
    button.click();
  });
}
bootstrap();
