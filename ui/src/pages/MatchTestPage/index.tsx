import MatchPlay from "../../components/MatchPlay";
import Roboticon2021Header from "../../components/Roboticon2021Header";

export default function MatchTestPage() {
    return (
        <div>
            <Roboticon2021Header />
            <div className="container">
                <h1>Match Play</h1>
                <MatchPlay />
            </div>
        </div>
    )
}