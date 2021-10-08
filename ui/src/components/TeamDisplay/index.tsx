import { useEffect } from "react"
import { Link } from "react-router-dom"
import { useLocation } from "react-router"
import { AllianceStation, useGetTeamAllianceStationsQuery, useRoboticonGameStateSubscription, useRoboticonScoresSubscription } from "../../generated/graphql"
import { GameState, GameType } from "../../roboticon"
import Roboticon2021Header from "../Roboticon2021Header"
import IdleDisplay from "../AudienceDisplay/idle"
import BasicTeamDisplay from "./basic"
import TeamIdleDisplay from "./teamidle"
import { whoNotReady } from "../../lib/whoNotReady"
import DancePartyTeamDisplay from "./danceparty"
import StunballTeamDisplay from "./stunball"

export default function TeamDisplay() {
    const station: AllianceStation = new URLSearchParams(useLocation().search).get("station") as AllianceStation
    const { data: stationsData, startPolling, stopPolling } = useGetTeamAllianceStationsQuery()

    useEffect(() => {
        startPolling(200)
        return () => {
            stopPolling()
        }
    })

    const { data: roboticonTickData } = useRoboticonGameStateSubscription()

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

    const showGameScreen = roboticonState.enabled || Object.keys(roboticonScores).length > 0

    if (station == null) {
        return (
            <div>
                <Roboticon2021Header />
                <div className="container">
                    <ul>
                        <li><Link to={`?station=${AllianceStation.Red_1}`}>RED 1</Link></li>
                        <li><Link to={`?station=${AllianceStation.Red_2}`}>RED 2</Link></li>
                        <li><Link to={`?station=${AllianceStation.Red_3}`}>RED 3</Link></li>
                        <li><Link to={`?station=${AllianceStation.Blue_1}`}>BLUE 1</Link></li>
                        <li><Link to={`?station=${AllianceStation.Blue_2}`}>BLUE 2</Link></li>
                        <li><Link to={`?station=${AllianceStation.Blue_3}`}>BLUE 3</Link></li>
                    </ul>
                </div>
            </div>
        )
    }

    let teamNum: number | null = null

    if (stationsData?.teamAllianceStations != null) {
        for (const s of stationsData.teamAllianceStations) {
            if (s.allianceStation === station) {
                teamNum = s.teamNumber
            }
        }
    }

    if (teamNum == null || (stationsData?.teamAllianceStations.length ?? 0) === 0) {
        return (
            <div style={{ cursor: "none" }}>
                <IdleDisplay />
            </div>
        )
    }

    if (!showGameScreen) {
        return (
            <div style={{ cursor: "none" }}>
                <TeamIdleDisplay
                    team={teamNum}
                    connected={!whoNotReady(stationsData!.teamAllianceStations, roboticonState.driverStationInfo).includes(teamNum)}
                    station={station}
                />
            </div>
        )
    }

    return (
        <div style={{ cursor: "none" }}>
            {[GameType.BASIC, GameType.SOCCER].includes(roboticonState.gameType) && (
                <BasicTeamDisplay
                    team={teamNum}
                    connected={!whoNotReady(stationsData!.teamAllianceStations, roboticonState.driverStationInfo).includes(teamNum)}
                    station={station}
                    scores={roboticonScores}
                    time={roboticonState.timeLeft}
                />
            )}
            {roboticonState.gameType === GameType.DANCEPARTY && (
                <DancePartyTeamDisplay
                    team={teamNum}
                    connected={!whoNotReady(stationsData!.teamAllianceStations, roboticonState.driverStationInfo).includes(teamNum)}
                    station={station}
                    scores={roboticonScores}
                    time={roboticonState.timeLeft}
                />
            )}
            {roboticonState.gameType === GameType.STUNBALL && (
                <StunballTeamDisplay
                    team={teamNum}
                    connected={!whoNotReady(stationsData!.teamAllianceStations, roboticonState.driverStationInfo).includes(teamNum)}
                    station={station}
                    scores={roboticonScores}
                    time={roboticonState.timeLeft}
                />
            )}
        </div>
    )
}