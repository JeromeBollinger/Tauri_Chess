const { invoke } = window.__TAURI__.tauri;

var canvas_length = window.screen.height/2;
var rect_length = canvas_length/8;


function fill_board(){
    let c = document.getElementById('board');
    c.width = canvas_length;
    c.height = canvas_length;
    let canvas = document.getElementById('board').getContext("2d");
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

window.addEventListener("load", () => {
    getBoard().then(
        board => {
            console.log(board);
            console.log(board.figures);
            board.figures.forEach((figure) => {
                drawFigure(figure);
                console.log(figure);
            });
        }).catch( error =>
            console.log(error, "could not fetch board!!! ")
    )
});

function drawFigure(figures) {
    let canvas = document.getElementById('board').getContext("2d");
    canvas.fillStyle = "yellow"
    let circle = new Path2D();
    circle.arc(figures.position.x * rect_length + rect_length / 2, figures.position.y * rect_length + rect_length / 2, rect_length / 2.5, 0, 2 * Math.PI);
    canvas.fill(circle);
    canvas.fillStyle = "black"
    canvas.font = "12px serif";
    canvas.fillText(figures.kind, figures.position.x * rect_length + rect_length / 4, figures.position.y * rect_length + rect_length / 2 + 5);
}
