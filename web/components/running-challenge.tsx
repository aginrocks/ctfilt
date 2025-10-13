import { RunningChallenge } from '@/types/server/RunningChallenge';
import { Icon, IconStopwatch, IconWorld } from '@tabler/icons-react';
import { useCountdown } from '@lib/hooks';
import { useModals } from '@lib/modals/manager';
import { ChallengeActions } from './challenge-view';
import { EXTEND_TRESHOLD_SECONDS } from '@lib/constants';

export type ChallengeLabelProps = {
    icon: Icon;
    label: string;
};

function ChallengeLabel({ icon: Icon, label }: ChallengeLabelProps) {
    return (
        <div className="flex items-center gap-1">
            <Icon className="text-muted-foreground size-3.5" />
            <div className="font-mono text-xs font-medium">{label}</div>
        </div>
    );
}

export function RunningChallengeBox(challenge: RunningChallenge) {
    const countdown = useCountdown(challenge.expires_at);
    const canExtend = countdown.seconds <= EXTEND_TRESHOLD_SECONDS;

    const modals = useModals();

    return (
        <div
            className="p-2 pt-2.5 border rounded-lg cursor-pointer hover:bg-secondary/30 transition-colors"
            onClick={() =>
                modals.show('Challenge', {
                    slug: challenge.slug,
                })
            }
        >
            {/*<div className="flex items-center gap-1 text-muted-foreground mb-1">
                <IconBox className="size-3.5" />
                <p className="text-xs font-medium">Challenge</p>
            </div>*/}
            <p className="font-medium text-sm mb-1.5">{challenge.name}</p>
            <div className="flex flex-col gap-1">
                <ChallengeLabel icon={IconStopwatch} label={countdown.formatted} />
                {challenge.status === 'running' && (
                    <ChallengeLabel icon={IconWorld} label={challenge.hostname ?? 'Unknown'} />
                )}
                <div className="flex gap-2 mt-1.5 w-full flex-1">
                    <ChallengeActions
                        canExtend={canExtend}
                        slug={challenge.slug}
                        variant="compact"
                        running={challenge}
                    />
                </div>
            </div>
        </div>
    );
}
