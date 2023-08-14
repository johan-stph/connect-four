function checkHorizontal(board: number[][], row: number, column: number, player: number): boolean {
    for (let i = 0; i < 4; i++) {
        if (column - i < 0 || column + 3 - i > 6) continue;
        if (board[row][column - i] === player &&
            board[row][column - i + 1] === player &&
            board[row][column - i + 2] === player &&
            board[row][column - i + 3] === player) {
            return true;
        }
    }
    return false;
}

function checkVertical(board: number[][], row: number, column: number, player: number): boolean {
    if (row + 3 < 6) {
        if (board[row][column] === player &&
            board[row + 1][column] === player &&
            board[row + 2][column] === player &&
            board[row + 3][column] === player) {
            return true;
        }
    }
    return false;
}

function checkDiagonalSlash(board: number[][], row: number, column: number, player: number): boolean {
    for (let i = 0; i < 4; i++) {
        if (row - (3 - i) >= 0 && row + i <= 5 && column - i >= 0 && column + (3 - i) <= 6) {
            let win = true;
            for (let j = 0; j < 4; j++) {
                if (board[row + i - j][column - i + j] != player) {
                    win = false;
                    break;
                }
            }
            if (win) {
                return true;
            }
        }
    }
    return false;
}

function checkDiagonalBackslash(board: number[][], row: number, column: number, player: number): boolean {
    for (let i = 0; i < 4; i++) {
        if (row - i >= 0 && row + (3 - i) <= 5 && column - i >= 0 && column + (3 - i) <= 6) {
            let win = true;
            for (let j = 0; j < 4; j++) {
                if (board[row - i + j][column - i + j] != player) {
                    win = false;
                    break;
                }
            }
            if (win) {
                return true;
            }
        }
    }
    return false;
}

export function checkForWin(board: number[][], row: number, column: number, player: number) {
    return checkHorizontal(board, row, column, player) ||
        checkVertical(board, row, column, player) ||
        checkDiagonalSlash(board, row, column, player) ||
        checkDiagonalBackslash(board, row, column, player);
}