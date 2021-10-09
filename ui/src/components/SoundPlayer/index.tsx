import React from "react";
import { useState } from "react";
import { useRoboticonGameStateSubscription, useRoboticonStunnedAlliancesSubscription } from "../../generated/graphql";
import { GameState, GameType, StunnedAlliances } from "../../roboticon";

//const abort = new Audio('/audio/abort.wav');
const end = new Audio('/audio/end.wav');
//const resume = new Audio('/audio/resume.wav');
const start = new Audio('/audio/start.wav');
const warning = new Audio('/audio/warning.wav');
const stunned = new Audio('/audio/stunned.mp3')

export default function SoundPlayer() {

    const [previouslyEnabled, setPreviouslyEnabled] = useState(false)
    const [previousTime, setPreviousTime] = useState(0)

    const [redPreviouslyStunned, setRedPreviouslyStunned] = useState(false)
    const [bluePreviouslyStunned, setBluePreviouslyStunned] = useState(false)

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



    const { data: stunData } = useRoboticonStunnedAlliancesSubscription()
    let stunnedAlliances: StunnedAlliances = {
        blueIsDisabled: false,
        blueIsInCooldown: false,
        redIsDisabled: false,
        redIsInCooldown: false
    }
    if (stunData?.subscribe != null) {
        stunnedAlliances = JSON.parse(stunData?.subscribe)
    }

    if ((stunnedAlliances.blueIsDisabled && !bluePreviouslyStunned) || (stunnedAlliances.redIsDisabled && !redPreviouslyStunned)) {
        stunned.play()
    }

    if (redPreviouslyStunned !== stunnedAlliances.redIsDisabled) setRedPreviouslyStunned(stunnedAlliances.redIsDisabled);
    if (bluePreviouslyStunned !== stunnedAlliances.blueIsDisabled) setBluePreviouslyStunned(stunnedAlliances.blueIsDisabled);



    return React.createElement(React.Fragment, null)
}