import { useEffect, useState } from "react"
import { useRoboticonStunnedAlliancesSubscription } from "../../generated/graphql"
import { StunnedAlliances } from "../../roboticon"
import Roboticon2021Header from "../Roboticon2021Header"
import styles from "./stunball.module.scss"

export function StunballDisplay({ redTeams, blueTeams, timeLeft, redScore, blueScore }: {
    redTeams: string[],
    blueTeams: string[],
    timeLeft: number,
    redScore: number,
    blueScore: number
}) {

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
        <div>
            <Roboticon2021Header />
            <div className={styles.timer}>
                <span>{timeLeft}</span>
            </div>
            <div className={styles.holder}>
                <div className={styles.red}>
                    <div className={styles.stunHolder}>
                        {stunnedAlliances.redIsDisabled && (
                            <div className={["card", styles.stunned].join(' ')}>
                                <h1>STUNNED!</h1>
                            </div>
                        )}
                    </div>
                    <h1 className={styles.score}>{redScore}</h1>
                    <div className={styles.teamHolder}>
                        {redTeams.map(t => (
                            <TeamCard key={t} num={t} />
                        ))}
                    </div>
                </div>
                <div className={styles.blue}>
                    <div className={styles.stunHolder}>
                        {stunnedAlliances.blueIsDisabled && (
                            <div className={["card", styles.stunned].join(' ')}>
                                <h1>STUNNED!</h1>
                            </div>
                        )}
                    </div>
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