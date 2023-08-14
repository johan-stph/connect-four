"use client"



import GameBoard from "../../components/gamboard/GameBoard";

export default function OfflinePage() {


    return (
        <div>
            <h1 className={"text-center text-3xl my-2.5"}>Connect Four</h1>
            <GameBoard/>
        </div>
    )
}