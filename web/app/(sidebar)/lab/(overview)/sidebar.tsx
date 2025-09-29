'use client';

import * as React from 'react';
import {
    IconBell,
    IconBook,
    IconBuildings,
    IconFlask,
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
    const courses = useQuery($api.queryOptions('get', '/api/courses'));

    return (
        <Sidebar variant="inset" {...props}>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <Logo />
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent className="gap-0">
                <NavMain
                    items={[
                        {
                            icon: IconBook,
                            title: 'Explore Courses',
                            url: '/lab/explore',
                        },
                    ]}
                    title="Lab"
                />
                <NavMain
                    title="Courses"
                    items={
                        courses.data?.map((c) => ({
                            icon: IconFlask,
                            title: c.name,
                            url: `/lab/courses/${c.slug}`,
                        })) || []
                    }
                />
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
