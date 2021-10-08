import { useEffect, useState } from "react"
import { useRoboticonReplyDanceMovesSubscription } from "../../generated/graphql"
import { DanceMove, DanceMoveToString } from "../../roboticon"
import Roboticon2021Header from "../Roboticon2021Header"
import styles from "./danceparty.module.scss"

export function DanceParty({ redTeams, blueTeams, timeLeft, scores }: {
    redTeams: string[],
    blueTeams: string[],
    timeLeft: number,
    scores: { [key: string]: number }
}) {

    const { data: danceMovesData } = useRoboticonReplyDanceMovesSubscription()
    let danceMoves: { [key: string]: DanceMove } = {}
    if (danceMovesData?.subscribe != null) {
        danceMoves = JSON.parse(danceMovesData.subscribe)
    }

    return (
        <div>
            <Roboticon2021Header />
            <div className={styles.timer}>
                <span>{timeLeft}</span>
            </div>
            <div className={styles.holder}>
                <div className={styles.red}>
                    {redTeams.map(t => (
                        <TeamCard key={t} num={t} score={scores[t] ?? 0} move={danceMoves[t]} />
                    ))}
                </div>
                <div className={styles.blue}>
                    {blueTeams.map(t => (
                        <TeamCard key={t} num={t} score={scores[t] ?? 0} move={danceMoves[t]} />
                    ))}
                </div>
            </div>
        </div>
    )
}

function TeamCard({ num, score, move }: { num: string, score: number, move?: DanceMove }) {
    const [teamName, setTeamName] = useState("")

    useEffect(() => {
        fetch(`https://www.thebluealliance.com/api/v3/team/frc${num}/simple`, {
            headers: {
                "X-TBA-Auth-Key": "MmmsCHZ2iRcb28PAM8FzNSY6VQWy3AxhPTfdfHQUqNEOAyy5Shgri4BEiDHKTGLH"
            }
        })
            .then(response => response.json())
            .then(data => setTeamName(data.nickname))
    }, [num])

    return (
        <div className={["card", styles.teamCard].join(' ')}>
            <h1>{score}</h1>
            <h4>Dance Move: {DanceMoveToString(move)}</h4>
            <h5>{num} | {teamName}</h5>
        </div>
    )
}