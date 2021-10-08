import { useState } from "react";
import { useRoboticonStunnedAlliancesSubscription, useRoboticonStunTeamMutation, useUpdateRoboticonScoreMutation } from "../../generated/graphql";
import { Alliance, StunnedAlliances, StunType } from "../../roboticon";
import Button from "../../styles/ohms-style/react/components/Button";
import Select from "../../styles/ohms-style/react/components/Select";
import styles from "./stunball.module.scss"

export default function StunballPanel({ scores }: { scores: { [key: string]: number } }) {
    const [alliance, setAlliance] = useState<Alliance | -1>(-1)
    const [updateScore] = useUpdateRoboticonScoreMutation()
    const [stunTeam] = useRoboticonStunTeamMutation()

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

    let cannotStun = false
    if (alliance === Alliance.RED) {
        if (stunnedAlliances.blueIsDisabled || stunnedAlliances.blueIsInCooldown) {
            cannotStun = true
        }
    } else if (alliance === Alliance.BLUE) {
        if (stunnedAlliances.redIsDisabled || stunnedAlliances.redIsInCooldown) {
            cannotStun = true
        }
    }

    return (
        <div>
            <div style={{ display: "flex" }}>
                <Select placeholder="Team" value={alliance} onChange={(e) => setAlliance(parseInt(e.target.value))}>
                    <option value={-1} disabled>Select an Alliance</option>
                    <option value={Alliance.RED}>RED</option>
                    <option value={Alliance.BLUE}>BLUE</option>
                </Select>
            </div>
            <div>
                {alliance >= 0 && (
                    <>
                        <h5>{alliance === Alliance.RED ? "Red" : "Blue"} score: {scores[alliance === Alliance.RED ? "1" : "0"]}</h5>
                        <div className={styles.buttons}>
                            <Button
                                large
                                variant="secondary"
                                disabled={cannotStun}
                                onClick={() => stunTeam({ variables: { data: JSON.stringify({ alliance: oppositeAlliance(alliance), stunType: StunType.LOWERGOAL }) } })}
                            >
                                Lower Goal
                            </Button>
                            <Button large
                                variant="secondary"
                                disabled={cannotStun}
                                onClick={() => stunTeam({ variables: { data: JSON.stringify({ alliance: oppositeAlliance(alliance), stunType: StunType.UPPERGOAL }) } })}
                            >
                                Outer Goal
                            </Button>
                            <Button large
                                variant="secondary"
                                disabled={cannotStun}
                                onClick={() => stunTeam({ variables: { data: JSON.stringify({ alliance: oppositeAlliance(alliance), stunType: StunType.INNERGOAL }) } })}
                            >
                                Inner Goal
                            </Button>
                        </div>
                        <div className={styles.buttons}>
                            <Button
                                large
                                variant="secondary"
                                onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: alliance === Alliance.RED ? 1 : 0, scoreDifference: +5 }) } })}
                            >
                                Hit Other Team
                            </Button>
                        </div>
                    </>
                )}
            </div>
        </div>
    )
}

function oppositeAlliance(alliance: Alliance): Alliance {
    if (alliance === Alliance.RED) {
        return Alliance.BLUE
    }
    return Alliance.RED
}