import { useEffect, useState } from "react"
import Roboticon2021Header from "../Roboticon2021Header"
import styles from "./basic-play.module.scss"

export function BasicPlay({ redTeams, blueTeams, timeLeft, redScore, blueScore }: {
    redTeams: string[],
    blueTeams: string[],
    timeLeft: number,
    redScore: number,
    blueScore: number
}) {
    return (
        <div>
            <Roboticon2021Header />
            <div className={styles.timer}>
                <span>{timeLeft}</span>
            </div>
            <div className={styles.holder}>
                <div className={styles.red}>
                    <h1 className={styles.score}>{redScore}</h1>
                    <div className={styles.teamHolder}>
                        {redTeams.map(t => (
                            <TeamCard key={t} num={t} />
                        ))}
                    </div>
                </div>
                <div className={styles.blue}>
                    <h1 className={styles.score}>{blueScore}</h1>
                    <div className={styles.teamHolder}>
                        {blueTeams.map(t => (
                            <TeamCard key={t} num={t} />
                        ))}
                    </div>
                </div>
            </div>
        </div>
    )
}

function TeamCard({ num }: { num: string }) {
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
            <h1>{num}</h1>
            <p>{teamName}</p>
        </div>
    )
}