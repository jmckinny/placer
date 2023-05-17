let boardLength;
function getBoard() {
    fetch("/api/v1/board")
        .then(response => {
            response.json().then(boardData => {
                displayBoard(boardData.tiles);
            })
        }).catch((err) => {
            console.error("Failed to get board with error: " + err);
        })

}

function displayBoard(board) {
    const canvas = document.getElementById("board-display");
    const ctx = canvas.getContext("2d");
    const width = canvas.width;

    boardLength = board.length;

    const size = width / boardLength;

    for (const [i, row] of board.entries()) {
        for (const [j, item] of row.entries()) {
            ctx.fillStyle = `rgb(${item.red},${item.green},${item.blue})`;
            ctx.fillRect(j * size, i * size, size, size);
        }
    }
}

function placeTile(row, col, rgb) {
    const { red, blue, green } = rgb;
    const data = {
        row: row,
        col: col,
        "tile": {
            "red": red,
            "blue": blue,
            "green": green
        }
    };
    fetch("/api/v1/place", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data)
    }).catch((res) => { console.error(res) })
}

getBoard();
const element = document.getElementById("board-display");
element.addEventListener("click", function (event) {
    const canvas = document.getElementById("board-display");
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    const size = canvas.width / boardLength;

    const row = Math.min(Math.floor(canvasTop / (size + 1)), canvas.height - 1);
    const col = Math.min(Math.floor(canvasLeft / (size + 1)), canvas.width - 1);

    const colorChoice = document.getElementById("color-choice").value;

    placeTile(row, col, hexToRGB(colorChoice));
    getBoard();
});

function hexToRGB(hex) {
    console.log(hex);
    const num = parseInt(hex.substring(1), 16);
    const red = (num >> 16) & 255;
    const green = (num >> 8) & 255;
    const blue = num & 255;
    return { red, green, blue };
}

setInterval(() => {
    getBoard();
}, 500);



