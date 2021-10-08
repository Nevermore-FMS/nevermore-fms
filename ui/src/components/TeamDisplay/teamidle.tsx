import { AllianceStation } from "../../generated/graphql";
import styles from "./teamidle.module.scss"

export default function TeamIdleDisplay({ station, connected, team }: { station: AllianceStation, connected: boolean, team: number }) {
    const isBlue = station.toString().includes("BLUE")
    return (
        <div className={`${isBlue ? styles.blue : styles.red}${connected ? "" : " " + styles.notconnected}`}>
            <h1>{team}</h1>
        </div>
    )
}