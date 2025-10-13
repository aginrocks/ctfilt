import { PulsingDot } from './ui/pulsing-dot';

export function PulsingDotDemo() {
    return (
        <div className="p-8 space-y-8">
            <h2 className="text-2xl font-bold mb-6">Pulsing Dot Component Demo</h2>

            {/* Basic Usage */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Basic Usage</h3>
                <div className="flex items-center gap-4">
                    <PulsingDot />
                    <span>Default green pulsing dot</span>
                </div>
            </section>

            {/* Different Sizes */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Sizes</h3>
                <div className="flex items-center gap-6">
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot size="xs" />
                        <span className="text-sm">XS</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot size="sm" />
                        <span className="text-sm">SM</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot size="md" />
                        <span className="text-sm">MD</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot size="lg" />
                        <span className="text-sm">LG</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot size="xl" />
                        <span className="text-sm">XL</span>
                    </div>
                </div>
            </section>

            {/* Different Colors */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Colors</h3>
                <div className="flex items-center gap-6">
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="green" />
                        <span className="text-sm">Green</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="red" />
                        <span className="text-sm">Red</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="blue" />
                        <span className="text-sm">Blue</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="yellow" />
                        <span className="text-sm">Yellow</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="orange" />
                        <span className="text-sm">Orange</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="purple" />
                        <span className="text-sm">Purple</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot color="gray" />
                        <span className="text-sm">Gray</span>
                    </div>
                </div>
            </section>

            {/* Different Variants */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Variants</h3>
                <div className="flex items-center gap-6">
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot variant="solid" size="lg" />
                        <span className="text-sm">Solid</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot variant="outline" size="lg" />
                        <span className="text-sm">Outline</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot variant="soft" size="lg" />
                        <span className="text-sm">Soft</span>
                    </div>
                </div>
            </section>

            {/* Different Speeds */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Animation Speeds</h3>
                <div className="flex items-center gap-6">
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot speed="slow" size="lg" />
                        <span className="text-sm">Slow</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot speed="normal" size="lg" />
                        <span className="text-sm">Normal</span>
                    </div>
                    <div className="flex flex-col items-center gap-2">
                        <PulsingDot speed="fast" size="lg" />
                        <span className="text-sm">Fast</span>
                    </div>
                </div>
            </section>

            {/* Static (No Animation) */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Static (No Animation)</h3>
                <div className="flex items-center gap-4">
                    <PulsingDot animate={false} />
                    <span>Static dot without animation</span>
                </div>
            </section>

            {/* Usage Examples */}
            <section className="space-y-4">
                <h3 className="text-lg font-semibold">Usage Examples</h3>
                <div className="space-y-3">
                    <div className="flex items-center gap-3 p-3 border rounded">
                        <PulsingDot color="green" size="sm" />
                        <span>Online status indicator</span>
                    </div>
                    <div className="flex items-center gap-3 p-3 border rounded">
                        <PulsingDot color="red" size="sm" />
                        <span>Error or alert indicator</span>
                    </div>
                    <div className="flex items-center gap-3 p-3 border rounded">
                        <PulsingDot color="blue" size="sm" speed="fast" />
                        <span>Active process indicator</span>
                    </div>
                    <div className="flex items-center gap-3 p-3 border rounded">
                        <PulsingDot color="yellow" size="sm" variant="outline" />
                        <span>Warning indicator</span>
                    </div>
                </div>
            </section>
        </div>
    );
}
