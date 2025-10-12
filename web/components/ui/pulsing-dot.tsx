import { cn } from '@/lib/utils';

interface PulsingDotProps {
    className?: string;
    size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
    color?: 'green' | 'red' | 'blue' | 'yellow' | 'orange' | 'purple' | 'gray';
    variant?: 'solid' | 'outline' | 'soft';
    animate?: boolean;
    speed?: 'slow' | 'normal' | 'fast';
}

function PulsingDot({
    className,
    size = 'md',
    color = 'green',
    variant = 'solid',
    animate = true,
    speed = 'normal',
}: PulsingDotProps) {
    const sizeClasses = {
        xs: 'size-1.5',
        sm: 'size-2',
        md: 'size-3',
        lg: 'size-4',
        xl: 'size-5',
    };

    const colorClasses = {
        green: {
            solid: 'bg-green-500',
            outline: 'border-green-500 bg-green-500/20',
            soft: 'bg-green-500/30',
        },
        red: {
            solid: 'bg-red-500',
            outline: 'border-red-500 bg-red-500/20',
            soft: 'bg-red-500/30',
        },
        blue: {
            solid: 'bg-blue-500',
            outline: 'border-blue-500 bg-blue-500/20',
            soft: 'bg-blue-500/30',
        },
        yellow: {
            solid: 'bg-yellow-500',
            outline: 'border-yellow-500 bg-yellow-500/20',
            soft: 'bg-yellow-500/30',
        },
        orange: {
            solid: 'bg-orange-500',
            outline: 'border-orange-500 bg-orange-500/20',
            soft: 'bg-orange-500/30',
        },
        purple: {
            solid: 'bg-purple-500',
            outline: 'border-purple-500 bg-purple-500/20',
            soft: 'bg-purple-500/30',
        },
        gray: {
            solid: 'bg-gray-500',
            outline: 'border-gray-500 bg-gray-500/20',
            soft: 'bg-gray-500/30',
        },
    };

    const pulseColorClasses = {
        green: 'bg-green-500',
        red: 'bg-red-500',
        blue: 'bg-blue-500',
        yellow: 'bg-yellow-500',
        orange: 'bg-orange-500',
        purple: 'bg-purple-500',
        gray: 'bg-gray-500',
    };

    const speedDurations = {
        slow: '3s',
        normal: '1s',
        fast: '0.5s',
    };

    const variantClasses = {
        solid: '',
        outline: 'border-2',
        soft: '',
    };

    const containerStyle = {
        '--pulse-duration': speedDurations[speed],
    } as React.CSSProperties;

    return (
        <div
            className={cn('relative inline-flex items-center justify-center', className)}
            style={containerStyle}
        >
            {/* Main dot */}
            <div
                className={cn(
                    'rounded-full z-10',
                    sizeClasses[size],
                    colorClasses[color][variant],
                    variantClasses[variant]
                )}
            />

            {/* Pulsing animation rings */}
            {animate && (
                <>
                    {/* Primary pulse ring */}
                    <div
                        className={cn(
                            'absolute inset-0 rounded-full opacity-75 animate-ping',
                            sizeClasses[size],
                            pulseColorClasses[color]
                        )}
                        style={{ animationDuration: 'var(--pulse-duration)' }}
                    />

                    {/* Secondary pulse ring for enhanced effect */}
                    <div
                        className={cn(
                            'absolute inset-0 rounded-full opacity-50 animate-ping',
                            sizeClasses[size],
                            pulseColorClasses[color]
                        )}
                        style={{
                            animationDuration: 'var(--pulse-duration)',
                            animationDelay: '0.5s',
                        }}
                    />
                </>
            )}
        </div>
    );
}

export { PulsingDot, type PulsingDotProps };
