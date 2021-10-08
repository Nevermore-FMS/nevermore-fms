import { useRoboticonScoresSubscription, useUpdateRoboticonScoreMutation } from "../../generated/graphql";
import { GameState } from "../../roboticon";
import Button from "../../styles/ohms-style/react/components/Button";
import styles from "./basic.module.scss"

export default function SoccerPanel({ state }: { state: GameState }) {

    const [updateScore] = useUpdateRoboticonScoreMutation()
    const { data: roboticonScoresData } = useRoboticonScoresSubscription()
    let roboticonScores: { [key: string]: number } = {}
    if (roboticonScoresData?.subscribe != null) { roboticonScores = JSON.parse(roboticonScoresData.subscribe) }

    return (
        <div>
            <h5>Red score: {roboticonScores["1"]}</h5>
            <div className={styles.redActions}>
                <Button large onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: 1, scoreDifference: +5 }) } })}>Red Goal</Button>
            </div>
            <h5>Blue score: {roboticonScores["0"]}</h5>
            <div className={styles.blueActions}>
                <Button large onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: 0, scoreDifference: +5 }) } })}>Blue Goal</Button>
            </div>
        </div>
    )
}