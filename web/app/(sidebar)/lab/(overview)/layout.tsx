import { SidebarInset } from '@components/ui/sidebar';
import { LabSidebar } from './sidebar';

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <LabSidebar />
            <SidebarInset>{children}</SidebarInset>
        </>
    );
}
