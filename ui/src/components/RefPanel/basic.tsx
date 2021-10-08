import { useUpdateRoboticonScoreMutation } from "../../generated/graphql";
import Button from "../../styles/ohms-style/react/components/Button";
import styles from "./basic.module.scss"

export default function BasicPanel({ scores }: { scores: { [key: string]: number } }) {

    const [updateScore] = useUpdateRoboticonScoreMutation()

    return (
        <div>
            <h5>Red score: {scores["1"]}</h5>
            <div className={styles.redActions}>
                <Button large onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: 1, scoreDifference: -1 }) } })}> - Red</Button>
                <Button large onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: 1, scoreDifference: +1 }) } })}> + Red</Button>
            </div>
            <h5>Blue score: {scores["0"]}</h5>
            <div className={styles.blueActions}>
                <Button large onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: 0, scoreDifference: -1 }) } })}> - Blue</Button>
                <Button large onClick={() => updateScore({ variables: { data: JSON.stringify({ teamNumber: 0, scoreDifference: +1 }) } })}> + Blue</Button>
            </div>
        </div>
    )
}