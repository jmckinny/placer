const SIZE = 12 // px
let BOARD_SIZE;
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


function hexToRGB(hex) {
    console.log(hex);
    const num = parseInt(hex.substring(1), 16);
    const red = (num >> 16) & 255;
    const green = (num >> 8) & 255;
    const blue = num & 255;
    return { red, green, blue };
}

function setSize() {
    const canvas = document.getElementById("board-display");
    fetch("/api/v1/size").then((res) => {
        res.json().then((data) => {
            BOARD_SIZE = Number(data);
            console.log("Board size " + BOARD_SIZE);
            canvas.height = (SIZE + 1) * BOARD_SIZE + 1;
            canvas.width = (SIZE + 1) * BOARD_SIZE + 1;
        })
    });
}

let boardLength;
function main() {

    setSize();

    getBoard();

    const element = document.getElementById("board-display");
    element.addEventListener("click", function (event) {
        const canvas = document.getElementById("board-display");
        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop = (event.clientY - boundingRect.top) * scaleY;

        const row = Math.min(Math.floor(canvasTop / (SIZE + 1)), canvas.height - 1);
        const col = Math.min(Math.floor(canvasLeft / (SIZE + 1)), canvas.width - 1);

        const colorChoice = document.getElementById("color-choice").value;

        placeTile(row, col, hexToRGB(colorChoice));
        getBoard();
    });

    setInterval(() => {
        getBoard();
    }, 500);
}

main()




