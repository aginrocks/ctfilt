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
import { useParams } from 'next/navigation';

export function CourseSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const { course_slug } = useParams<{ course_slug: string }>();
    const course = useQuery(
        $api.queryOptions('get', '/api/courses/{course_slug}', {
            params: {
                path: { course_slug },
            },
        })
    );

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
                {/*<NavMain
                    title="Lessons"
                    items={
                        course.data?.items.map((c, i) => ({
                            icon: IconFlask,
                            title: c.lesson.$oid || c.challenge.$oid,
                            url: `/lab/courses/${i}`,
                        })) || []
                    }
                />*/}
                <NavSecondary items={navSecondary} className="mt-auto" />
            </SidebarContent>
            <SidebarFooter>
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
