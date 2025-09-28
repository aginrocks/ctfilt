'use client';

import * as React from 'react';
import {
    IconBell,
    IconBuildings,
    IconHistory,
    IconHome,
    IconServer,
    IconSettings,
    IconUsers,
} from '@tabler/icons-react';
import { NavMain, NavMainSubItem } from '@/components/nav-main';
import { NavSecondary } from '@/components/nav-secondary';
import { NavUser } from '@/components/nav-user';
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { Logo } from '@components/logo';
import { navSecondary } from '@components/sidebar-common';

export function LabSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    return (
        <Sidebar variant="inset" {...props}>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <Logo />
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent>
                <NavMain
                    items={[
                        {
                            icon: IconHome,
                            title: 'Home',
                            url: '/app',
                        },
                        {
                            icon: IconBell,
                            title: 'Notifications',
                            url: '/app/notifications',
                        },
                        {
                            icon: IconBuildings,
                            title: 'Organizations',
                            url: '/app/orgs',
                            // defaultOpen: true,
                        },
                    ]}
                    title="General"
                />
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
