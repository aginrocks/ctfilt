import { paths } from '@/types/api';
import MarkdownRenderer from './markdown';
import { IconClockPlus, IconPlayerPlayFilled, IconPlayerStop } from '@tabler/icons-react';
import { Button, buttonVariants } from './ui/button';
import { useRunning } from '@lib/atoms';
import { useExtendChallenge, useStartChallenge, useStopChallenge } from '@lib/mutations';
import { useCallback, useMemo } from 'react';
import { Spinner } from './ui/spinner';
import { Copyable } from './copyable';
import { useCountdown } from '@lib/hooks';
import { FlagInput } from './flag-input';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';
import { EXTEND_TRESHOLD_SECONDS } from '@lib/constants';
import { RunningChallenge } from '@/types/server/RunningChallenge';
import { VariantProps } from 'class-variance-authority';
import { MouseEvent } from 'react';
import { useAtomValue } from 'jotai';
import { VpnDevices } from '@lib/atoms/vpn-devices';

export type ChallengeViewProps = {
    challenge: paths['/api/challenges/{challenge_slug}']['get']['responses']['200']['content']['application/json'];
};

export function ExtendTooltip({ canExtend }: { canExtend: boolean }) {
    return canExtend ? (
        'Extend challenge time'
    ) : (
        <div className="w-50 py-0.5">
            <div className="font-bold text-sm">Unable to Extend</div>
            <div>
                You can only extend the challenge if less than
                <b> {EXTEND_TRESHOLD_SECONDS / 60} minutes </b>
                remain
            </div>
        </div>
    );
}

export function ChallengeActions({
    running,
    canExtend,
    slug,
    variant = 'full',
}: {
    running?: RunningChallenge;
    canExtend: boolean;
    slug: string;
    variant?: 'full' | 'compact';
}) {
    const start = useStartChallenge();
    const stop = useStopChallenge();
    const extend = useExtendChallenge();
    const startChallenge = useCallback(
        (e: MouseEvent) => {
            e.stopPropagation();
            start.mutate({
                params: {
                    path: {
                        challenge_slug: slug,
                    },
                },
            });
        },
        [start.mutate, slug]
    );

    const stopChallenge = useCallback(
        (e: MouseEvent) => {
            e.stopPropagation();
            stop.mutate({
                params: {
                    path: {
                        challenge_slug: slug,
                    },
                },
            });
        },
        [stop.mutate, slug]
    );

    const extendChallenge = useCallback(
        (e: MouseEvent) => {
            e.stopPropagation();
            extend.mutate({
                params: {
                    path: {
                        challenge_slug: slug,
                    },
                },
            });
        },
        [extend.mutate, slug]
    );

    const fullButtonProps: Partial<VariantProps<typeof buttonVariants>> & { className?: string } =
        variant === 'full' ? { size: 'lg' } : { size: 'default', className: 'rounded-sm w-full' };

    return running ? (
        <>
            {running.status === 'starting' && (
                <Button {...fullButtonProps} variant="lightOrange">
                    <Spinner /> Starting Challenge
                </Button>
            )}
            {running.status === 'stopping' && (
                <Button {...fullButtonProps} variant="lightOrange">
                    <Spinner /> Stopping Challenge
                </Button>
            )}
            {running.status === 'running' && (
                <>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                size={variant === 'full' ? 'icon-lg' : 'default'}
                                variant="lightRed"
                                onClick={stopChallenge}
                                className={variant === 'compact' ? 'flex-1 rounded-sm' : ''}
                            >
                                {stop.isPending ? <Spinner /> : <IconPlayerStop />}
                                {variant === 'compact' && ' Stop Challenge'}
                            </Button>
                        </TooltipTrigger>
                        {variant !== 'compact' && <TooltipContent>Stop challenge</TooltipContent>}
                    </Tooltip>
                    {variant !== 'compact' && (
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <div>
                                    <Button
                                        size="icon-lg"
                                        variant="secondary"
                                        onClick={extendChallenge}
                                        disabled={!canExtend || extend.isPending}
                                    >
                                        {extend.isPending ? <Spinner /> : <IconClockPlus />}
                                    </Button>
                                </div>
                            </TooltipTrigger>
                            <TooltipContent>
                                <ExtendTooltip canExtend={canExtend} />
                            </TooltipContent>
                        </Tooltip>
                    )}
                    {variant !== 'compact' && <FlagInput challengeSlug={slug} />}
                </>
            )}
        </>
    ) : (
        <Button {...fullButtonProps} onClick={startChallenge}>
            {start.isPending ? <Spinner /> : <IconPlayerPlayFilled />} Start Challenge
        </Button>
    );
}

export function ChallengeView({ challenge }: ChallengeViewProps) {
    const running = useRunning(challenge._id);
    const devices = useAtomValue(VpnDevices);

    const remaining = useCountdown(running?.expires_at);
    const canExtend = remaining.seconds <= EXTEND_TRESHOLD_SECONDS;

    const vpnDevice = useMemo(
        () => devices?.find((d) => d.hostname === running?.hostname),
        [devices, running?.hostname]
    );

    return (
        <div>
            <div className="mb-3">
                <h1 className="scroll-m-20 text-3xl font-bold tracking-tight text-balance mb-2">
                    {challenge.name}
                </h1>
                <p className="italic font-medium text-muted-foreground">{challenge.description}</p>
            </div>
            <MarkdownRenderer>{challenge.details}</MarkdownRenderer>
            <div className="flex flex-col gap-3 mt-4">
                {running?.status === 'running' && (
                    <div className="px-3 py-2.5 border rounded-md flex gap-4">
                        <div className="flex-3">
                            <div className="font-medium text-xs text-muted-foreground mb-0.5">
                                Hostname
                            </div>
                            <Copyable value={running.hostname} />
                        </div>
                        <div className="flex-2">
                            <div className="font-medium text-xs text-muted-foreground mb-0.5">
                                IP Address
                            </div>
                            <Copyable
                                value={running.ip || vpnDevice?.ip_addresses[0] || 'Loading...'}
                            />
                        </div>
                        <div className="flex-2">
                            <div className="font-medium text-xs text-muted-foreground mb-0.5">
                                Expires in
                            </div>
                            <h3 className="font-lg font-medium font-mono hover:foreground/90">
                                {remaining.formatted}
                            </h3>
                        </div>
                    </div>
                )}
                <div className="flex gap-2.5">
                    <ChallengeActions
                        running={running}
                        canExtend={canExtend}
                        slug={challenge.slug}
                    />
                </div>
            </div>
        </div>
    );
}
