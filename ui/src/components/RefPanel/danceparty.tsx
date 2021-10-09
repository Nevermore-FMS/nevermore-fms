import { useState } from "react";
import { useRoboticonNewDanceMoveMutation, useRoboticonReplyDanceMovesSubscription, useUpdateRoboticonScoreMutation } from "../../generated/graphql";
import { DanceMove, DanceMoveToString } from "../../roboticon";
import Button from "../../styles/ohms-style/react/components/Button";
import Select from "../../styles/ohms-style/react/components/Select";

export default function DancePartyPanel({ teams }: { teams: number[] }) {
    const [team, setTeam] = useState<number>(-1)
    const [updateScore] = useUpdateRoboticonScoreMutation()
    const [newDanceMove] = useRoboticonNewDanceMoveMutation()

    const { data: danceMovesData } = useRoboticonReplyDanceMovesSubscription()
    let danceMoves: { [key: string]: DanceMove } = {}
    if (danceMovesData?.subscribe != null) {
        danceMoves = JSON.parse(danceMovesData.subscribe)
    }

    return (
        <div>
            <div style={{ display: "flex" }}>
                <Select placeholder="Team" value={team} onChange={(e) => setTeam(parseInt(e.target.value))}>
                    <option value={-1} disabled>Select a team</option>
                    {teams.map(t => (
                        <option key={t} value={t}>{t}</option>
                    ))}
                </Select>
            </div>
            <div>
                {team > 0 && (
                    <div>
                        <h2>Current Dance Move: {DanceMoveToString(danceMoves[team.toString()])}</h2>
                        <Button large variant="secondary" onClick={() => {
                            updateScore({ variables: { data: JSON.stringify({ teamNumber: team, scoreDifference: +5 }) } })
                            newDanceMove({ variables: { data: JSON.stringify(team) } })
                        }}>Mark Dance Move Completed</Button>
                    </div>
                )}
            </div>
        </div>
    )
}