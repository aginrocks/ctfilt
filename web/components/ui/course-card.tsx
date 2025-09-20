import { Icon } from '@tabler/icons-react';
import { LucideIcon } from 'lucide-react';

export type CourseCardProps = {
    icon: LucideIcon | Icon;
    title: string;
    description: string;
    progress: number;
};

export function CourseCard({ title, description, icon: Icon }: CourseCardProps) {
    return (
        <div
            className="rounded-lg border cursor-pointer overflow-hidden p-4 w-96 shadow-sm bg-secondary/30"
            // style={{
            //     background: `linear-gradient(45deg,#ffffff00,${primaryColor})`,
            // }}
        >
            <div className="shadow-md rounded-lg border w-10 h-10 flex justify-center items-center bg-secondary">
                <Icon className="size-5" />
            </div>
            <h3 className="text-xl font-medium mb-1 mt-2">{title}</h3>
            <p className="text-sm text-muted-foreground">{description}</p>
        </div>
    );
}
