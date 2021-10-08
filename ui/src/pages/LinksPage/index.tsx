import { Link } from "react-router-dom";
import Roboticon2021Header from "../../components/Roboticon2021Header";

export default function LinksPage() {
    return (
        <div>
            <Roboticon2021Header />
            <div className="container">
                <h1>Pages:</h1>
                <ul>
                    <li><Link to="/control">Game Control Panel</Link></li>
                    <li><Link to="/sounds">Sound Player</Link></li>
                    <li><Link to="/audience">Audience Display</Link></li>
                    <li><Link to="/refpanel">Referee Panel</Link></li>
                </ul>
            </div>
        </div>
    )
}