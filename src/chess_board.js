const canvas_length = 320;
const rect_length = canvas_length/8;


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
