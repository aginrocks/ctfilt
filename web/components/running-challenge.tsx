import { RunningChallenge } from '@/types/server/RunningChallenge';
import { Icon, IconStopwatch, IconWorld } from '@tabler/icons-react';
import { useCountdown } from '@lib/hooks';
import { useModals } from '@lib/modals/manager';

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

export function RunningChallengeBox({ hostname, expires_at }: RunningChallenge) {
    const countdown = useCountdown(expires_at);

    const modals = useModals();

    return (
        <div
            className="p-2.5 border rounded-md cursor-pointer hover:bg-secondary bg-secondary/80 transition-colors"
            onClick={() =>
                modals.show('Challenge', {
                    slug: 'traversing-trust',
                })
            }
        >
            {/*<div className="flex items-center gap-1 text-muted-foreground mb-1">
                <IconBox className="size-3.5" />
                <p className="text-xs font-medium">Challenge</p>
            </div>*/}
            <p className="font-medium text-sm mb-1.5">Challenge Name</p>
            <div className="flex flex-col gap-1">
                <ChallengeLabel icon={IconStopwatch} label={countdown.formatted} />
                <ChallengeLabel icon={IconWorld} label={hostname ?? 'Unknown'} />
            </div>
        </div>
    );
}
