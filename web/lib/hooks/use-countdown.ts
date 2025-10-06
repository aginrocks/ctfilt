import { useState, useEffect } from 'react';
import moment, { Moment } from 'moment';

interface CountdownResult {
    formatted: string;
    diff: moment.Duration;
    isOver: boolean;
}

function deriveCountdown(to: Moment | null): CountdownResult | null {
    if (!to) return null;

    const diff = moment.duration(to.diff(moment()));
    const isOver = diff.asMilliseconds() <= 0;

    let formatted = '';
    if (diff.asHours() >= 1) {
        formatted += `${Math.floor(diff.asHours())}h `;
    }
    formatted += `${diff.minutes().toString().padStart(2, '0')}m ${diff.seconds().toString().padStart(2, '0')}s`;

    return {
        formatted,
        diff,
        isOver: diff.asMilliseconds() <= 0,
    };
}

export function useCountdown(toStr?: string): string {
    const to = toStr ? moment(toStr) : null;
    const [timeRemaining, setTimeRemaining] = useState(() => deriveCountdown(to));

    useEffect(() => {
        if (!to) return;

        const interval = setInterval(() => {
            const countdown = deriveCountdown(to);
            setTimeRemaining(countdown);

            if (countdown?.isOver) {
                clearInterval(interval);
            }
        }, 1000);

        return () => clearInterval(interval);
    }, [to]);

    return timeRemaining?.formatted ?? '';
}
