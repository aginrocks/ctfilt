import { useClipboard } from '@mantine/hooks';
import { Button } from './ui/button';
import clsx from 'clsx';
import { IconCheck, IconCopy } from '@tabler/icons-react';

export type CopyableProps = {
    value: string | null;
    // Defaults to `value`
    copyValue?: string | null;
};

export function Copyable({ value, copyValue }: CopyableProps) {
    const { copy, copied } = useClipboard();
    return (
        <div
            className="flex items-center gap-0.5 cursor-pointer"
            onClick={() => copy(copyValue ?? value)}
        >
            <h3 className="font-lg font-medium font-mono hover:foreground/90">{value}</h3>
            <Button
                size="icon-xs"
                variant="ghost"
                className={clsx('mt-0.5 transition-none', { '!text-green-300': copied })}
            >
                {copied ? <IconCheck /> : <IconCopy />}
            </Button>
        </div>
    );
}
