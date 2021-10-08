import RefPanel from "../../components/RefPanel";
import Roboticon2021Header from "../../components/Roboticon2021Header";

export default function RefPanelPage() {
    return (
        <div>
            <Roboticon2021Header />
            <div className="container">
                <h1>Referee Panel</h1>
                <RefPanel />
            </div>
        </div>
    )
}