import { AllianceStation, useRoboticonStunnedAlliancesSubscription } from "../../generated/graphql";
import { StunnedAlliances } from "../../roboticon";
import styles from "./stunball.module.scss"


export default function StunballTeamDisplay({ station, connected, team, scores, time }: { time: number, station: AllianceStation, connected: boolean, team: number, scores: { [key: string]: number } }) {
    const isBlue = station.toString().includes("BLUE")
    const score = scores[team.toString()] ?? 0

    const { data: stunData } = useRoboticonStunnedAlliancesSubscription()
    let stunnedAlliances: StunnedAlliances = {
        blueIsDisabled: false,
        blueIsInCooldown: false,
        redIsDisabled: false,
        redIsInCooldown: false
    }
    if (stunData?.subscribe != null) {
        stunnedAlliances = JSON.parse(stunData?.subscribe)
    }

    return (
        <div className={`${isBlue ? styles.blue : styles.red} ${(isBlue ? stunnedAlliances.blueIsDisabled : stunnedAlliances.redIsDisabled) ? styles.stunned : ""}`}>
            <h3>{time}</h3>
            <h1>{score}</h1>
            <h3>{team}</h3>
        </div>
    )
}