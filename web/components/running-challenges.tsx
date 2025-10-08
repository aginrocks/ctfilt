import { RunningChallenges } from '@lib/atoms';
import { useAtomValue } from 'jotai';
import { RunningChallengeBox } from './running-challenge';

export function RunningChallengesView() {
    const challenges = useAtomValue(RunningChallenges);

    if (challenges?.length === 0) return null;
    return (
        <div>
            {challenges?.map((c) => (
                <RunningChallengeBox key={c._id} {...c} />
            ))}
        </div>
    );
}
