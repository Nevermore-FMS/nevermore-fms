import { useEffect, useState } from "react"
import Roboticon2021Header from "../Roboticon2021Header"
import styles from "./matchup.module.scss"

export function Matchup({ redTeams, blueTeams }: { redTeams: string[], blueTeams: string[] }) {
    return (
        <div>
            <Roboticon2021Header />
            <div className={styles.holder}>
                <div className={styles.red}>
                    {redTeams.map(t => (
                        <TeamCard key={t} num={t} />
                    ))}
                </div>
                <div className={styles.blue}>
                    {blueTeams.map(t => (
                        <TeamCard key={t} num={t} />
                    ))}
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