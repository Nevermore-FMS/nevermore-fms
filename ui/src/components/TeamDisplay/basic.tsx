import { AllianceStation } from "../../generated/graphql";
import styles from "./basic.module.scss"


export default function BasicTeamDisplay({ station, connected, team, scores, time }: { time: number, station: AllianceStation, connected: boolean, team: number, scores: { [key: string]: number } }) {
    const isBlue = station.toString().includes("BLUE")
    const score = scores[isBlue ? "0" : "1"] ?? 0
    return (
        <div className={`${isBlue ? styles.blue : styles.red}${connected ? "" : " " + styles.notconnected}`}>
            <h3>{time}</h3>
            <h1>{score}</h1>
            <h2>{team}</h2>
        </div>
    )
}