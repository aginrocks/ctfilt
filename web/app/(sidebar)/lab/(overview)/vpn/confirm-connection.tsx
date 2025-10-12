import { PulsingDot } from '@components/ui/pulsing-dot';

export function ConfirmConnection() {
    return (
        <div className="border rounded-lg p-4 w-full mt-2.5 flex justify-center items-center gap-2.5">
            <PulsingDot speed="slow" color="red" />
            <p className="font-medium">Waiting for Connection</p>
        </div>
    );
}
