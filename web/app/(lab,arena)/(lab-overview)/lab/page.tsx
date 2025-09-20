import { AppSidebar } from '@/components/app-sidebar';
import { PageHeader } from '@/components/page-header';
import { SidebarInset } from '@/components/ui/sidebar';
import { LabOverviewNav } from './sidebar';
import { CourseCard } from '@components/ui/course-card';
import { IconNetwork } from '@tabler/icons-react';

export default function Page() {
    return (
        <>
            <AppSidebar>
                <LabOverviewNav />
            </AppSidebar>
            <SidebarInset>
                <PageHeader
                    path={[
                        {
                            label: 'Lab',
                        },
                    ]}
                />
                <div className="flex flex-1 flex-col gap-4 px-4 py-10 items-center">
                    <div className="w-full max-w-4xl">
                        <h1 className="text-3xl font-bold mb-2">Welcome to the Lab!</h1>
                        <p className="text-muted-foreground">
                            Master your skills with hands-on challenges and guided learning
                        </p>
                    </div>
                    <CourseCard
                        icon={IconNetwork}
                        title="Networking Basics"
                        description="Learn the basics of networking"
                        progress={10}
                    />
                </div>
            </SidebarInset>
        </>
    );
}
