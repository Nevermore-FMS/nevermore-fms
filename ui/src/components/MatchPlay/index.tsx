import { useApolloClient } from "@apollo/client"
import { useEffect, useState } from "react"
import { AddTeamToFieldDocument, AddTeamToFieldMutationVariables, AllianceStation, RemoveTeamFromFieldDocument, RemoveTeamFromFieldMutationVariables, useGetTeamAllianceStationsQuery } from "../../generated/graphql"
import Button from "../../styles/ohms-style/react/components/Button"
import TextField from "../../styles/ohms-style/react/components/TextField"
import styles from "./index.module.scss"

export default function MatchPlay() {
    const client = useApolloClient()
    const { data: stationsData, startPolling, stopPolling } = useGetTeamAllianceStationsQuery()

    useEffect(() => {
        startPolling(200)
        return () => {
            stopPolling()
        }
    })

    const finalized = (stationsData?.teamAllianceStations?.length ?? 0) > 0

    const defaultInputs: { [key: string]: string } = {
        "RED_1": "",
        "RED_2": "",
        "RED_3": "",
        "BLUE_1": "",
        "BLUE_2": "",
        "BLUE_3": "",
    }
    const [inputs, setInputs] = useState(defaultInputs)

    const getValue = (station: string): string => {
        if (stationsData?.teamAllianceStations == null) return ""
        if (finalized) {
            for (const stat of stationsData.teamAllianceStations) {
                if (stat.allianceStation === station) {
                    return stat.teamNumber.toString()
                }
            }
            return ""
        } else {
            return inputs[station]
        }
    }

    const onInput = (station: string, value: string) => {
        if (stationsData?.teamAllianceStations == null) return ""
        const newInputs = {...inputs}
        if (!finalized) {
            newInputs[station] = value
        }
        setInputs(newInputs)
    }

    const textFields = (stations: string[]) => stations.map((v) => (
        <TextField
            key={v}
            placeholder={v.replace("_", " ")}
            type="number"
            disabled={finalized}
            value={getValue(v)}
            onChange={(e) => onInput(v, e.target.value)} />
    ))

    const clearTeams = () => {
        if (stationsData?.teamAllianceStations == null) return
        for (const stat of stationsData.teamAllianceStations) {
            client.mutate<any, RemoveTeamFromFieldMutationVariables>({
                mutation: RemoveTeamFromFieldDocument,
                variables: {
                    teamNumber: stat.teamNumber
                }
            })
        }
    }

    const setTeams = () => {
        for (const station of Object.keys(inputs)) {
            const teamNum = parseInt(inputs[station])
            if (!isNaN(teamNum)) {
                console.log(teamNum)
                client.mutate<any, AddTeamToFieldMutationVariables>({
                    mutation: AddTeamToFieldDocument,
                    variables: {
                        allianceStation: station as AllianceStation,
                        teamNumber: teamNum
                    }
                })
            }
        }
        setInputs(defaultInputs)
    }

    return (
        <div>
            <div className={styles.teamsHolder}>
                <div className={styles.redTeams}>
                    {textFields(["RED_1", "RED_2", "RED_3"])}
                </div>
                <div className={styles.blueTeams}>
                    {textFields(["BLUE_1", "BLUE_2", "BLUE_3"])}
                </div>
            </div>
            <div className={styles.actionsHolder}>
                {finalized && (
                    <Button variant="primary" large onClick={() => clearTeams()}>Clear Teams</Button>
                )}
                {!finalized && (
                    <Button variant="secondary" large onClick={() => setTeams()}>Setup Match</Button>
                )}
            </div>
        </div>
    )
}