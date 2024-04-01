const { invoke } = window.__TAURI__.tauri;
const canva = document.getElementById("board");

var canvas_length = window.screen.height/2;
var rect_length = canvas_length/8;


function fill_board(){
    canva.width = canvas_length;
    canva.height = canvas_length;
    let canvas = canva.getContext("2d");
    let n = 0;
    for (let x = 0; x <= 8; x++) {
        for (let y = 0; y <= 8; y++) {
            if (n%2 == 0){
                canvas.fillStyle = "black"
            } else {
                canvas.fillStyle = "white"
            }
            canvas.fillRect(x * rect_length, y * rect_length, canvas_length, canvas_length)
            n++;
        }
    }
}

document.addEventListener("load", fill_board());

async function getBoard() {
    let figures = await invoke("get_board");
    return figures;
}

let figureShapes

window.addEventListener("load", () => {
    getBoard().then(
        board => {
            figureShapes = drawFigures(board);
        }).catch( error =>
            console.log(error, "could not fetch board!!! ")
    )
});


canva.addEventListener('click', e => {
    let canvas = canva.getContext("2d");
    figureShapes.forEach((figureShape) => {
        if (canvas.isPointInPath(figureShape.shape, e.offsetX, e.offsetY)) {
            console.log(figureShape.object.kind)
        }
    })
})

function drawFigures(board){
    var figureShapes = [];
    board.figures.forEach((figure) => {
        figureShapes.push(drawFigure(figure));
    });
    return figureShapes;
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
    return {"shape": circle, "object": figure};
}
