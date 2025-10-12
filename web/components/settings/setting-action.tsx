import { cn } from '@lib/utils';
import { Icon } from '@tabler/icons-react';
import { cva, VariantProps } from 'class-variance-authority';
import React from 'react';

export type SettingActionProps = React.ComponentProps<'div'> & {
    title?: string;
    description?: string;
    rightSection?: React.ReactNode;
    icon?: Icon;
    clickable?: boolean;
    selected?: boolean;
};

export const variants = cva('', {
    variants: {
        layout: {
            default: 'flex gap-3 items-center',
            vertical: 'flex flex-col gap-3 [&_svg]:size-7',
        },
    },
});

export function SettingAction({
    title,
    description,
    rightSection,
    className,
    icon: Icon,
    children,
    clickable = false,
    selected = false,
    layout = 'default',
    ...props
}: SettingActionProps & VariantProps<typeof variants>) {
    return (
        <div
            className={cn(
                'p-4 rounded-lg flex justify-between items-center border mt-4 gap-2',
                clickable && 'cursor-pointer hover:bg-card/50',
                selected && 'border-transparent outline-2 outline-primary -outline-offset-1',
                layout === 'vertical' && 'flex-1',
                className
            )}
            {...props}
        >
            <div className={cn(variants({ layout }))}>
                {Icon && <Icon className="size-6 text-muted-foreground" />}
                <div className="flex-1 flex flex-col gap-0.5">
                    {title && <div className="font-medium">{title}</div>}
                    {description && (
                        <div className="text-xs text-muted-foreground">{description}</div>
                    )}
                    {children}
                </div>
            </div>
            <div>{rightSection}</div>
        </div>
    );
}
