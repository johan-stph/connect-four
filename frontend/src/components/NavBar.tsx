import {FaGithub} from "react-icons/fa";
import Link from "next/link";

export default function NavBar() {
    return (
        <nav className={""}>
            <div className={"flex justify-between items-center max-w-6xl mx-auto px-4 py-4"}>
                <div className={"flex items-center text-xl font-semibold"}>
                    <Link href={"/"}>Home</Link>
                </div>
                <div className={"flex"}>
                    <ul>
                        <li className={"inline-block mx-4 text-xl font-semibold"}>
                            <Link href={"/game"}>Play</Link>
                        </li>
                        <li className={"inline-block mx-4 text-xl font-semibold"}>
                            <Link href={"/analyze"}>Analyse</Link>
                        </li>
                        <li className={"inline-block mx-4 text-xl font-semibold"}>
                            <Link href={"/online"}>Online</Link>
                        </li>
                    </ul>
                </div>
                <div className={"flex items-center"}>
                    <a href={"https://github.com/johan-stph/connect-four"}
                       className={"mx-5"}
                       target={"_blank"}>
                        <FaGithub size={25}/></a>
                </div>

            </div>
        </nav>
    )

}