import Roboticon2021Header from "../../components/Roboticon2021Header";
import SoundPlayer from "../../components/SoundPlayer";

export default function SoundsPage() {
    return (
        <div>
            <Roboticon2021Header />
            <div className="container">
                <h1>Sound Player</h1>
                <SoundPlayer />
            </div>
        </div>
    )
}