import { cn } from '@lib/utils';
import React from 'react';

export function Tag({ children, className, ...props }: React.ComponentProps<'div'>) {
    return (
        <div
            className={cn(
                "px-3.5 py-2 rounded-full bg-secondary/50 text-secondary-foreground text-sm font-medium max-w-max mb-3 flex items-center gap-1.5 [&_svg:not([class*='size-'])]:size-4",
                className
            )}
            {...props}
        >
            {children}
        </div>
    );
}
