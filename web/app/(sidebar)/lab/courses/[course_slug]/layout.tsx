import { SidebarInset } from '@components/ui/sidebar';
import { CourseSidebar } from './sidebar';

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <CourseSidebar />
            <SidebarInset>{children}</SidebarInset>
        </>
    );
}
