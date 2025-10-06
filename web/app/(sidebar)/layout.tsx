import { SidebarProvider } from '@components/ui/sidebar';
import { AtomsBindProvider } from '@lib/providers/atoms-bind';

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <SidebarProvider>
            <AtomsBindProvider>{children}</AtomsBindProvider>
        </SidebarProvider>
    );
}
