const { invoke } = window.__TAURI__.tauri;
const canva = document.getElementById("board");

var canvas_length = window.screen.height / 2;
var rect_length = canvas_length / 8;


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

async function getBoard() {
  let figures = await invoke("get_board");
  return figures;
}

let figureShapes = [];
let optionShapes = [];

window.addEventListener("load", () => {
  redrawBoard()
});

async function getOptions(figureId) {
  let options = await invoke("get_options", { "figureId": figureId });
  return options;
}

canva.addEventListener('click', e => {
  let canvas = canva.getContext("2d");
  let figureId;
  figureShapes.forEach((figureShape) => {
    if (canvas.isPointInPath(figureShape.shape, e.offsetX, e.offsetY)) {
      console.log(figureShape.object)
      figureId = figureShape.object.id;

      getOptions(figureId).then(
        options => {
          optionShapes = drawOptions(options.movable)
        }
      ).catch(error =>
        console.log(error, "could not fetch options")
      );
    }
  })

})

function drawFigures(board) {
  board.figures.forEach((figure) => {
    figureShapes.push(drawFigure(figure));
  });
}

function drawFigure(figure) {
  let canvas = canva.getContext("2d");
  canvas.fillStyle = "yellow"
  let circle = new Path2D();
  circle.arc(figure.position.x * rect_length + rect_length / 2, figure.position.y * rect_length + rect_length / 2, rect_length / 2.5, 0, 2 * Math.PI);
  canvas.fill(circle);
  canvas.fillStyle = "black"
  canvas.font = "12px serif";
  canvas.fillText(figure.kind, figure.position.x * rect_length + rect_length / 4, figure.position.y * rect_length + rect_length / 2 + 5);
  return { "shape": circle, "object": figure };
}

function drawOptions(options) {
  var optionShapes = [];
  options.forEach((option) => {
    optionShapes.push(drawOption(option));
  });
  return optionShapes;
}

function drawOption(option) {
  let canvas = canva.getContext("2d");
  canvas.fillStyle = "orange"
  let circle = new Path2D();
  circle.arc(option.x * rect_length + rect_length / 2, option.y * rect_length + rect_length / 2, rect_length / 5, 0, 2 * Math.PI);
  canvas.fill(circle);
  return { "shape": circle, "object": option };
}

function clearBoard() {
  canva.getContext("2d").clearRect(0, 0, canva.width, canva.height);
}

function clearShapes() {
  figureShapes = []
}

function redrawBoard() {
  clearBoard();
  clearShapes();
  fillBoard();
  getBoard().then(
    board => {
      drawFigures(board)
    }).catch(error =>
      console.log(error, "could not fetch board!!! ")
    )
}
