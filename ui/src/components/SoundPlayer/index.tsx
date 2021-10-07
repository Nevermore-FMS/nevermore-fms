import React from "react";
import { useState } from "react";
import { useRoboticonGameStateSubscription } from "../../generated/graphql";
import { GameState, GameType } from "../../roboticon";

//const abort = new Audio('/audio/abort.wav');
const end = new Audio('/audio/end.wav');
//const resume = new Audio('/audio/resume.wav');
const start = new Audio('/audio/start.wav');
const warning = new Audio('/audio/warning.wav');

export default function SoundPlayer() {

    const [previouslyEnabled, setPreviouslyEnabled] = useState(false)
    const [previousTime, setPreviousTime] = useState(0)

    const { data: roboticonTickData } = useRoboticonGameStateSubscription()
    let roboticonState: GameState = { 
        driverStationInfo: [],
        eStopped: false,
        enabled: false,
        gameType: GameType.BASIC,
        timeLeft: 0
     }
    if (roboticonTickData?.subscribe != null) { roboticonState = JSON.parse(roboticonTickData.subscribe) }

    if (!previouslyEnabled && roboticonState.enabled) {
        start.play()
    } else if (previouslyEnabled && !roboticonState.enabled) {
        end.play()
    } else if (roboticonState.enabled && previousTime > 30 && roboticonState.timeLeft <= 30) {
        warning.play()
    }

    if (previouslyEnabled !== roboticonState.enabled) setPreviouslyEnabled(roboticonState.enabled)
    if (previousTime !== roboticonState.timeLeft) setPreviousTime(roboticonState.timeLeft)

    return React.createElement(React.Fragment, null)
}