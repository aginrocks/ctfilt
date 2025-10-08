import { paths } from '@/types/api';
import MarkdownRenderer from './markdown';
import {
    IconBox,
    IconCheck,
    IconClockPlus,
    IconCopy,
    IconPlayerPlayFilled,
    IconPlayerStop,
    IconSend,
} from '@tabler/icons-react';
import { Button } from './ui/button';
import { useRunning } from '@lib/atoms';
import { useExtendChallenge, useStartChallenge, useStopChallenge } from '@lib/mutations';
import { useCallback } from 'react';
import { Spinner } from './ui/spinner';
import { useClipboard } from '@mantine/hooks';
import clsx from 'clsx';
import { Copyable } from './copyable';
import { useCountdown } from '@lib/hooks';
import { FlagInput } from './flag-input';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';

export type ChallengeViewProps = {
    challenge: paths['/api/challenges/{challenge_slug}']['get']['responses']['200']['content']['application/json'];
};

export function ChallengeView({ challenge }: ChallengeViewProps) {
    const running = useRunning(challenge._id);
    const start = useStartChallenge();
    const stop = useStopChallenge();
    const extend = useExtendChallenge();

    const remaining = useCountdown(running?.expires_at);
    const canExtend = remaining.seconds <= 30 * 60;

    const startChallenge = useCallback(() => {
        start.mutate({
            params: {
                path: {
                    challenge_slug: challenge.slug,
                },
            },
        });
    }, [start.mutate, challenge.slug]);

    const stopChallenge = useCallback(() => {
        stop.mutate({
            params: {
                path: {
                    challenge_slug: challenge.slug,
                },
            },
        });
    }, [stop.mutate, challenge.slug]);

    const extendChallenge = useCallback(() => {
        extend.mutate({
            params: {
                path: {
                    challenge_slug: challenge.slug,
                },
            },
        });
    }, [extend.mutate, challenge.slug]);

    return (
        <div>
            <div className="mb-3">
                <h1 className="scroll-m-20 text-3xl font-bold tracking-tight text-balance mb-2">
                    {challenge.name}
                </h1>
                <p className="italic font-medium text-muted-foreground">{challenge.description}</p>
            </div>
            <MarkdownRenderer>{challenge.details}</MarkdownRenderer>
            {/*{running?.status === 'running' && (
                <div className="mt-4">
                    <FlagInput challengeSlug={challenge.slug} />
                </div>
            )}*/}
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
                            <Copyable value={running.ip || 'Unknown'} />
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
                    {running ? (
                        <>
                            {running.status === 'starting' && (
                                <Button size="lg" variant="lightOrange">
                                    <Spinner /> Starting Challenge
                                </Button>
                            )}
                            {running.status === 'stopping' && (
                                <Button size="lg" variant="lightOrange">
                                    <Spinner /> Stopping Challenge
                                </Button>
                            )}
                            {running.status === 'running' && (
                                <>
                                    <Tooltip>
                                        <TooltipTrigger asChild>
                                            <Button
                                                size="icon-lg"
                                                variant="lightRed"
                                                onClick={stopChallenge}
                                            >
                                                {stop.isPending ? <Spinner /> : <IconPlayerStop />}
                                            </Button>
                                        </TooltipTrigger>
                                        <TooltipContent>Stop challenge</TooltipContent>
                                    </Tooltip>
                                    <Tooltip>
                                        <TooltipTrigger asChild>
                                            <div>
                                                <Button
                                                    size="icon-lg"
                                                    variant="secondary"
                                                    onClick={extendChallenge}
                                                    disabled={!canExtend || extend.isPending}
                                                >
                                                    {extend.isPending ? (
                                                        <Spinner />
                                                    ) : (
                                                        <IconClockPlus />
                                                    )}
                                                </Button>
                                            </div>
                                        </TooltipTrigger>
                                        <TooltipContent>
                                            {canExtend ? (
                                                'Extend challenge time'
                                            ) : (
                                                <div className="w-50 py-0.5">
                                                    <div className="font-bold text-sm">
                                                        Unable to Extend
                                                    </div>
                                                    <div>
                                                        You can only extend the challenge if less
                                                        than
                                                        <b> 30 minutes </b>remain
                                                    </div>
                                                </div>
                                            )}
                                        </TooltipContent>
                                    </Tooltip>
                                    <FlagInput challengeSlug={challenge.slug} />
                                </>
                            )}
                        </>
                    ) : (
                        <Button size="lg" onClick={startChallenge}>
                            {start.isPending ? <Spinner /> : <IconPlayerPlayFilled />} Start
                            Challenge
                        </Button>
                    )}
                </div>
            </div>
        </div>
    );
}
