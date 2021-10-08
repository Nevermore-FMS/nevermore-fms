import { useEffect } from "react"
import { useEStopRoboticonGameMutation, useGetTeamAllianceStationsQuery, useRoboticonGameStateSubscription, useRoboticonScoresSubscription } from "../../generated/graphql"
import { GameState, GameType } from "../../roboticon"
import Button from "../../styles/ohms-style/react/components/Button"
import BasicPanel from "./basic"
import DancePartyPanel from "./danceparty"
import SoccerPanel from "./soccer"
import StunballPanel from "./stunball"

export default function RefPanel() {
    const { data: stationsData, startPolling, stopPolling } = useGetTeamAllianceStationsQuery()

    useEffect(() => {
        startPolling(200)
        return () => {
            stopPolling()
        }
    })

    const { data: roboticonTickData } = useRoboticonGameStateSubscription()

    const [eStopRoboticonGame] = useEStopRoboticonGameMutation()

    let roboticonState: GameState = {
        driverStationInfo: [],
        eStopped: false,
        enabled: false,
        gameType: GameType.BASIC,
        timeLeft: 0
    }
    if (roboticonTickData?.subscribe != null) { roboticonState = JSON.parse(roboticonTickData.subscribe) }

    const { data: roboticonScoresData } = useRoboticonScoresSubscription()
    let roboticonScores: { [key: string]: number } = {}
    if (roboticonScoresData?.subscribe != null) { roboticonScores = JSON.parse(roboticonScoresData.subscribe) }

    return (
        <div>
            {roboticonState.eStopped && (
                <Button variant="secondary" large onClick={() => eStopRoboticonGame({ variables: { eStop: "false" } })}>Stop EStop</Button>
            )}
            {!roboticonState.eStopped && (
                <Button disabled={!roboticonState.enabled} variant="secondary" large onClick={() => eStopRoboticonGame({ variables: { eStop: "true" } })}>EStop All</Button>
            )}
            <h3>Time left: {roboticonState.timeLeft}</h3>
            {roboticonState.gameType === GameType.BASIC && (
                <BasicPanel scores={roboticonScores} />
            )}
            {roboticonState.gameType === GameType.SOCCER && (
                <SoccerPanel scores={roboticonScores} />
            )}
            {roboticonState.gameType === GameType.DANCEPARTY && (
                <DancePartyPanel teams={(stationsData?.teamAllianceStations ?? []).map(a => a.teamNumber)} />
            )}
            {roboticonState.gameType === GameType.STUNBALL && (
                <StunballPanel scores={roboticonScores} />
            )}
        </div>
    )
}