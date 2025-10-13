import { cn } from '@lib/utils';

export type StepProps = {
    number: number;
    title: string;
} & React.ComponentProps<'div'>;

export function Step({ number, title, className, children, ...props }: StepProps) {
    return (
        <div className={cn('flex gap-3', className)} {...props}>
            <div className="bg-secondary text-secondary-foreground size-8 min-w-8 rounded-full flex items-center justify-center text-lg font-bold">
                {number}
            </div>
            <div className="flex-1">
                <div className="flex items-center h-8">
                    <h3 className="font-semibold text-lg">{title}</h3>
                </div>
                {children}
            </div>
        </div>
    );
}
