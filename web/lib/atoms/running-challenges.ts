import { RunningChallenge } from '@/types/server/RunningChallenge';
import { useSocketEvent } from '@lib/hooks';
import { atom, useAtomValue, useSetAtom } from 'jotai';
import { useCallback, useContext } from 'react';

export const RunningChallenges = atom<RunningChallenge[]>();

export function useBindChallenges() {
    console.log('bidning');
    const setRunningChallenges = useSetAtom(RunningChallenges);

    useSocketEvent(
        'challenges-update',
        useCallback((message) => {
            setRunningChallenges(message.challenges);
        }, [])
    );
}

export function useRunning(id: string) {
    const challenges = useAtomValue(RunningChallenges);
    const challenge = challenges?.find((c) => c._id === id);
    return challenge;
}
