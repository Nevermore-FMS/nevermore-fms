import { useEffect } from "react"
import { useEStopRoboticonGameMutation, useGetTeamAllianceStationsQuery, useRoboticonGameStateSubscription } from "../../generated/graphql"
import { GameState, GameType } from "../../roboticon"
import Button from "../../styles/ohms-style/react/components/Button"
import BasicPanel from "./basic"
import SoccerPanel from "./soccer"

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
                <BasicPanel state={roboticonState} />
            )}
            {roboticonState.gameType === GameType.SOCCER && (
                <SoccerPanel state={roboticonState} />
            )}
        </div>
    )
}