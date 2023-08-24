import {FaGithub} from "react-icons/fa";
import Link from "next/link";

export default function NavBar() {
    return (
        <nav className={""}>
            <div className={"flex justify-between items-center max-w-6xl mx-auto px-4 py-4"}>
                <div className={"flex items-center text-xl font-semibold"}>
                    <Link href={"/"}>Home</Link>
                </div>
                <div className={"flex justify-between items-center mx-4 text-xl font-semibold"}>
                    Connect Four
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