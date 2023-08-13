import React, {useState} from "react";
import {QueryClient, QueryClientProvider, useQuery} from "@tanstack/react-query";
import {checkForWin} from "@/components/gamboard/logic";


async function getEvaluation(boardString: string) {
    const url = "api/connect-four/?position=" + boardString;
    const res = await fetch(url);
    if (!res.ok) {
        return "Backend not available";
    }
    return res.text();
}

const ConnectFour: React.FC = () => {
    const queryClient = new QueryClient()
    const initialBoard = Array(6)
        .fill(0)
        .map(() => Array(7).fill(0));
    const [board, setBoard] = useState<number[][]>(initialBoard);
    const [isRedTurn, setIsRedTurn] = useState(true);
    const [winner, setWinner] = useState<string | null>(null);
    const [boardString, setBoardString] = useState<string>("");
    const [showEvaluation, setShowEvaluation] = useState(false);


    function resetGame() {
        setBoard(initialBoard);
        setIsRedTurn(true);
        setWinner(null);
        setBoardString("");
    }

    function handleClick(colIndex: number, rowIndex: number) {
        if (winner || board[rowIndex][colIndex] !== 0) {
            return;
        }
        let found = false;
        for (let i = board.length - 1; i >= 0; i--) {
            if (board[i][colIndex] === 0) {
                rowIndex = i;
                found = true;
                break;
            }
        }
        if (!found) {
            return;
        }
        const newBoard = [...board];
        newBoard[rowIndex][colIndex] = isRedTurn ? 1 : -1;
        setBoardString((prev) => prev + (colIndex + 1));

        // Check for win
        if (checkForWin(newBoard, rowIndex, colIndex, isRedTurn ? 1 : -1)) {
            setWinner(isRedTurn ? "Red" : "Blue");
            return;
        }
        setBoard(newBoard);
        setIsRedTurn(!isRedTurn);
    }

    return (
        <div className="flex flex-col items-center justify-center space-y-2">
            {winner && (
                <div className="modal">
                    <h2>{winner} wins!</h2>
                    <button onClick={resetGame}>Play Again</button>
                </div>
            )}

            {board.map((row, rowIndex) => (
                <div key={rowIndex} className="flex justify-center space-x-2">
                    {row.map((cell, colIndex) => {
                        let discColor = "";
                        if (cell) {
                            discColor = cell === 1 ? "bg-red-500" : "bg-blue-500";
                        }
                        return (
                            <div
                                key={`cell-${rowIndex}-${colIndex}`}
                                className="border border-black w-12 h-12 flex items-center justify-center rounded-full cursor-pointer m-2"
                                onClick={() => handleClick(colIndex, rowIndex)}
                            >
                                {(cell === 1 || cell === -1) && (
                                    <div
                                        className={`w-10 h-10 rounded-full ${discColor}`}
                                    ></div>
                                )}
                            </div>
                        );
                    })}
                </div>
            ))}
            <button onClick={() => setShowEvaluation(prev => !prev)}>
                Toggle Evaluation
            </button>
            {showEvaluation && (
                <div className="flex justify-center">
                    <div>
                        <QueryClientProvider client={queryClient}>
                            <Evaluation boardString={boardString}/>
                        </QueryClientProvider>
                    </div>
                </div>
            )}
        </div>
    );
};


function Evaluation({boardString}: { boardString: string }) {
    const {isLoading, isError, data, error} = useQuery({
        queryKey: ["connectfour", boardString],
        queryFn: () => getEvaluation(boardString),
    })

    if (isLoading) {
        return <p>Loading...</p>
    }
    if (isError) {
        // @ts-ignore
        return <p>Error: {error.message}</p>
    }
    return <p>{data}</p>
}




export default ConnectFour;
