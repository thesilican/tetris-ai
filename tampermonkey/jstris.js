(function bootstrap(callback) {
  let game = null;
  window.getGame = () => game;

  // Hook into game.start()
  const originalStart = window.Game.prototype.start;
  window.Game.prototype.start = function (...args) {
    game = this;
    console.log(game);
    originalStart.apply(this, args);
    callback(game);
  };

  // Hook into game.restart()
  const originalRestart = window.Game.prototype.restart;
  window.Game.prototype.restart = function (...args) {
    originalRestart.apply(this, args);
    game.applyGravityLvl(0);
    game.setLockDelay([Infinity, Infinity, Infinity]);
  };

  // Never change focus
  window.Game.prototype.browserTabFocusChange = function () {};
})(function (game) {
  // Game Inputs
  const KEY = {
    SHIFT_LEFT: () => game.Settings.controls[0],
    SHIFT_RIGHT: () => game.Settings.controls[1],
    SOFT_DROP: () => game.Settings.controls[2],
    HARD_DROP: () => game.Settings.controls[3],
    ROT_LEFT: () => game.Settings.controls[4],
    ROT_RIGHT: () => game.Settings.controls[5],
    ROT_180: () => game.Settings.controls[7],
    HOLD: () => game.Settings.controls[6],
  };
  function gameKeyDown(key) {
    game.keyInput2({
      keyCode: key(),
      preventDefault: () => {},
      stopPropagation: () => {},
    });
  }
  function gameKeyUp(key) {
    game.keyInput3({
      keyCode: key(),
      preventDefault: () => {},
      stopPropagation: () => {},
    });
  }
  function gameKeyPress(key) {
    gameKeyDown(key);
    gameKeyUp(key);
  }
  // Game outputs
  const PIECES = new Map([
    [0, 1],
    [1, 0],
    [2, 2],
    [3, 3],
    [4, 4],
    [5, 5],
    [6, 6],
  ]);
  function getGameState() {
    const current = PIECES.get(game.activeBlock.id);
    const hold =
      game.blockInHold === null ? null : PIECES.get(game.blockInHold.id);
    const queue = game.queue.map((x) => PIECES.get(x));
    const matrix = [];
    for (let i = 0; i < 10; i++) {
      const col = [];
      for (let j = 0; j < 20; j++) {
        const gameCell = game.matrix[20 - j - 1][i];
        col.push(gameCell === 0);
      }
      matrix.push(col);
    }
    return {
      current,
      hold,
      queue,
      matrix,
    };
  }

  // Build buttons
  const $container = $(`<div class="injected-container"></div>`);
  $container.css("position", "absolute");
  $container.css("top", "0");
  $container.css("left", "0");
  $container.css("padding-top", "108px");
  const $start = $("<button>Start</button>");
  $container.append($start);
  $("#main").append($container);

  $start.click(() => {});
});
