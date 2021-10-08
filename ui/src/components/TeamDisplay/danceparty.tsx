import { AllianceStation, useRoboticonReplyDanceMovesSubscription } from "../../generated/graphql";
import { DanceMove, DanceMoveToString } from "../../roboticon";
import styles from "./danceparty.module.scss"


export default function DancePartyTeamDisplay({ station, connected, team, scores, time }: { time: number, station: AllianceStation, connected: boolean, team: number, scores: { [key: string]: number } }) {
    const isBlue = station.toString().includes("BLUE")
    const score = scores[team.toString()] ?? 0

    const { data: danceMovesData } = useRoboticonReplyDanceMovesSubscription()
    let danceMoves: { [key: string]: DanceMove } = {}
    if (danceMovesData?.subscribe != null) {
        danceMoves = JSON.parse(danceMovesData.subscribe)
    }
    return (
        <div className={`${isBlue ? styles.blue : styles.red}${connected ? "" : " " + styles.notconnected}`}>
            <h3>{time}</h3>
            <h2>{DanceMoveToString(danceMoves[team.toString()])}</h2>
            <h1>{score}</h1>
            <h3>{team}</h3>
        </div>
    )
}