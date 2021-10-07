import { useApolloClient } from "@apollo/client"
import { useEffect, useState } from "react"
import { AddTeamToFieldDocument, AddTeamToFieldMutationVariables, AllianceStation, RemoveTeamFromFieldDocument, RemoveTeamFromFieldMutationVariables, useGetTeamAllianceStationsQuery, useRoboticonGameStateSubscription, useStartRoboticonGameMutation, useStopRoboticonGameMutation, useSwitchRoboticonGameMutation } from "../../generated/graphql"
import Button from "../../styles/ohms-style/react/components/Button"
import TextField from "../../styles/ohms-style/react/components/TextField"
import styles from "./index.module.scss"
import { GameState, GameType } from "../../roboticon"
import Select from "../../styles/ohms-style/react/components/Select"

export default function MatchPlay() {
    const client = useApolloClient()
    const { data: stationsData, startPolling, stopPolling } = useGetTeamAllianceStationsQuery()

    useEffect(() => {
        startPolling(200)
        return () => {
            stopPolling()
        }
    })

    const { data: roboticonTickData } = useRoboticonGameStateSubscription()

    const [startRoboticonGame] = useStartRoboticonGameMutation()
    const [stopRoboticonGame] = useStopRoboticonGameMutation()
    const [switchRoboticonGame] = useSwitchRoboticonGameMutation()

    let roboticonState: GameState = {
        driverStationInfo: [],
        eStopped: false,
        enabled: false,
        gameType: GameType.BASIC,
        timeLeft: 0
    }
    if (roboticonTickData?.subscribe != null) { roboticonState = JSON.parse(roboticonTickData.subscribe) }

    const finalized = (stationsData?.teamAllianceStations?.length ?? 0) > 0
    const allReady = true

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
        const newInputs = { ...inputs }
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
            <div className={styles.headerOptions}>
                <Select placeholder="Game Type" value={roboticonState.gameType} onChange={(e) => switchRoboticonGame({ variables: { game: e.target.value } })}>
                    <option value={GameType.BASIC}>Basic</option>
                    <option value={GameType.DANCEPARTY}>Dance Party</option>
                    <option value={GameType.STUNBALL}>Stunball</option>
                    <option value={GameType.SOCCER}>Soccer</option>
                </Select>
            </div>
            <div className={styles.header}>
                <div className={["card", styles.statusCard].join(' ')}>
                    {!finalized && (
                        <h1>Awaiting Setup</h1>
                    )}
                    {(finalized && !allReady) && (
                        <h1>Teams Not Ready</h1>
                    )}
                    {(finalized && allReady && !roboticonState.enabled) && (
                        <h1>Ready to Start</h1>
                    )}
                    {(roboticonState.enabled) && (
                        <h1>{Math.round(roboticonState.timeLeft)}</h1>
                    )}
                </div>
            </div>
            <div className={styles.teamsHolder}>
                <div className={styles.redTeams}>
                    {textFields(["RED_1", "RED_2", "RED_3"])}
                </div>
                <div className={styles.blueTeams}>
                    {textFields(["BLUE_1", "BLUE_2", "BLUE_3"])}
                </div>
            </div>
            <div className={styles.actionsHolder}>
                {!finalized && (
                    <Button variant="secondary" large onClick={() => setTeams()}>Setup Match</Button>
                )}
                {finalized && !roboticonState.enabled && (
                    <Button variant="primary" large onClick={() => clearTeams()}>Clear Teams</Button>
                )}
                {finalized && allReady && !roboticonState.enabled && (
                    <Button variant="secondary" large onClick={() => startRoboticonGame()}>Start Match</Button>
                )}
                {roboticonState.enabled && (
                    <Button variant="primary" large onClick={() => stopRoboticonGame()}>Stop Match</Button>
                )}
            </div>
        </div>
    )
}