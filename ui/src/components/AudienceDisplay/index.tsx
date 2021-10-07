import { useEffect } from "react";
import { useGetTeamAllianceStationsQuery, useRequestRoboticonScoresMutation, useRoboticonGameStateSubscription, useRoboticonScoresSubscription } from "../../generated/graphql";
import { GameState, GameType } from "../../roboticon";
import { BasicPlay } from "./basic-play";
import IdleDisplay from "./idle";
import { Matchup } from "./matchup";

export default function AudienceDisplay() {

    const { data: stationsData, startPolling, stopPolling } = useGetTeamAllianceStationsQuery()

    const [requestRoboticonScores] = useRequestRoboticonScoresMutation()

    useEffect(() => {
        startPolling(200)
        requestRoboticonScores()
        return () => {
            stopPolling()
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    const { data: roboticonTickData } = useRoboticonGameStateSubscription()

    let roboticonState: GameState = {
        driverStationInfo: [],
        eStopped: false,
        enabled: false,
        gameType: GameType.BASIC,
        timeLeft: 0
    }
    if (roboticonTickData?.subscribe != null) { roboticonState = JSON.parse(roboticonTickData.subscribe) }

    const finalized = (stationsData?.teamAllianceStations?.length ?? 0) > 0

    const { data: roboticonScoresData } = useRoboticonScoresSubscription()
    let roboticonScores: { [key: string]: number } = {}
    if (roboticonScoresData?.subscribe != null) { roboticonScores = JSON.parse(roboticonScoresData.subscribe) }
    console.log(roboticonScores)

    return (
        <div style={{ overflowY: "hidden" }}>
            {!finalized && <IdleDisplay />}
            {finalized && !roboticonState.enabled && (
                <Matchup
                    redTeams={stationsData!!.teamAllianceStations.filter(a => a.allianceStation.toString().includes("RED")).map(a => a.teamNumber.toString())}
                    blueTeams={stationsData!!.teamAllianceStations.filter(a => a.allianceStation.toString().includes("BLUE")).map(a => a.teamNumber.toString())} />
            )}
            {finalized && roboticonState.enabled && roboticonState.gameType === GameType.BASIC && (
                <BasicPlay
                    redTeams={stationsData!!.teamAllianceStations.filter(a => a.allianceStation.toString().includes("RED")).map(a => a.teamNumber.toString())}
                    blueTeams={stationsData!!.teamAllianceStations.filter(a => a.allianceStation.toString().includes("BLUE")).map(a => a.teamNumber.toString())}
                    blueScore={roboticonScores["0"] ?? 0}
                    redScore={roboticonScores["1"] ?? 0}
                    timeLeft={Math.round(roboticonState.timeLeft)}
                />
            )}
        </div>
    )
}