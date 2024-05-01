const { invoke } = window.__TAURI__.tauri;
var canva = document.getElementById("board");
var canva2 = document.getElementById("board2");

var canvas_length = window.screen.height / 2;
var rect_length = canvas_length / 8;
var first_canva = true;

window.addEventListener("load", () => {
  redrawBoard()
});

canva.addEventListener('click', e => {
  let position = { 'x': Math.floor(e.offsetX / rect_length), 'y': Math.floor(e.offsetY / rect_length) }

  positionInteraction(position).then(
    _ => {})
  redrawBoard();
      getOptions(position).then(
        options => {
          if (options !== null) {
            drawOptions(options.movable, "orange")
            drawOptions(options.killable, "red")
          }
        }
      ).catch(error =>
        console.log(error, "could not fetch options or not correct turn")
      );
})

canva2.addEventListener('click', e => {
  let position = { 'x': Math.floor(e.offsetX / rect_length), 'y': Math.floor(e.offsetY / rect_length) }

  positionInteraction(position).then(
    _ => {})
  redrawBoard();
      getOptions(position).then(
        options => {
          if (options !== null) {
            drawOptions(options.movable, "orange")
            drawOptions(options.killable, "red")
          }
        }
      ).catch(error =>
        console.log(error, "could not fetch options or not correct turn")
      );
})

// Rust invokes
async function positionInteraction(position){
  let c = await invoke("position_interaction", position);
  return c;
}

async function getBoard() {
  let figures = await invoke("get_board");
  return figures;
}

async function getOptions(position) {
  let options = await invoke("get_options", position);
  return options;
}


// Drawing to canvas
function drawFigures(board) {
  board.figures.forEach((figure) => {
    if (figure.alive){
      drawFigure(figure);
    }
  });
}

function drawFigure(figure) {
  let color = figure.white ? "#FFDAB9" : "#8B5742";
  let circle = drawCircle(color, figure.position.x, figure.position.y, rect_length / 2.5);
  let canvas = canva.getContext("2d");
  canvas.fillStyle = "black"
  canvas.font = "12px serif";
  canvas.fillText(figure.kind, figure.position.x * rect_length + rect_length / 4, figure.position.y * rect_length + rect_length / 2 + 5);
  return { "shape": circle, "object": figure };
}

function drawOptions(options, color) {
  let optionShapes = [];
  options.forEach((option) => {
    optionShapes.push({"shape": drawCircle(color, option.x, option.y, rect_length /5), "object": option})
  });
  return optionShapes;
}

function clearBoard() {
  canva2.getContext("2d").clearRect(0, 0, canva.width, canva.height);
}

function redrawBoard() {
  var c = canva;
  canva = canva2;
  canva2 = c;
  fillBoard();
  getBoard().then(
    board => {
      drawFigures(board)
    }).catch(error =>
      console.log(error, "could not fetch board!!! ")
    )
  canva.classList.remove("hidden");
  canva2.classList.add("hidden");
  clearBoard();
}

function fillBoard() {
  canva.width = canvas_length;
  canva.height = canvas_length;
  let canvas = canva.getContext("2d");
  let n = 0;
  for (let x = 0; x <= 8; x++) {
    for (let y = 0; y <= 8; y++) {
      if (n % 2 == 0) {
        canvas.fillStyle = "black"
      } else {
        canvas.fillStyle = "white"
      }
      canvas.fillRect(x * rect_length, y * rect_length, canvas_length, canvas_length)
      n++;
    }
  }
}

function drawCircle(fillstyle, x, y, size){
  let canvas = canva.getContext("2d");
  canvas.fillStyle = fillstyle
  let circle = new Path2D();
  circle.arc(x * rect_length + rect_length / 2, y * rect_length + rect_length / 2, size, 0, 2 * Math.PI);
  canvas.fill(circle);
  return circle;
}
