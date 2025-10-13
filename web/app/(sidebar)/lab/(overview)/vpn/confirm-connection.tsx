import { Button } from '@components/ui/button';
import { PulsingDot } from '@components/ui/pulsing-dot';
import { useRecentlyConnected } from '@lib/atoms/vpn-devices';
import { IconArrowRight, IconCheck, IconCircleCheck } from '@tabler/icons-react';
import clsx from 'clsx';

export function ConfirmConnection() {
    const device = useRecentlyConnected();

    return (
        <>
            <div
                className={clsx('border rounded-lg p-4 w-full mt-2.5', {
                    'flex justify-center items-center gap-2.5': !device,
                })}
            >
                {device ? (
                    <>
                        <IconCircleCheck className="text-green-500 size-8 mb-2" />
                        <p className="text-lg font-bold">{device.name}</p>
                        <div className="flex items-center gap-2.5 mt-1 ml-0.5">
                            <PulsingDot speed="slow" color="green" size="sm" />
                            <p className="font-medium text-sm text-muted-foreground">Connected</p>
                        </div>
                    </>
                ) : (
                    <>
                        <PulsingDot speed="slow" color="red" />
                        <p className="font-medium">Waiting for Connection</p>
                    </>
                )}
            </div>
            {device && (
                <div className="flex justify-end">
                    <Button className="mt-4" size="lg">
                        Go to Dashboard <IconArrowRight />
                    </Button>
                </div>
            )}
        </>
    );
}
