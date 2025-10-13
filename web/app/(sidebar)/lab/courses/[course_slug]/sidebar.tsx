'use client';

import * as React from 'react';
import {
    IconArrowNarrowLeft,
    IconBell,
    IconBook,
    IconBook2,
    IconBox,
    IconBuildings,
    IconFlask,
    IconHistory,
    IconHome,
    IconPlayerPlayFilled,
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
    SidebarGroup,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { Logo } from '@components/logo';
import { navSecondary } from '@components/sidebar-common';
import { useParams } from 'next/navigation';
import { useAtomValue } from 'jotai';
import { RunningChallenges } from '@lib/atoms';
import clsx from 'clsx';
import { Button } from '@components/ui/button';
import Link from 'next/link';
import { RunningChallengesView } from '@components/running-challenges';

export function CourseSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    const { course_slug } = useParams<{ course_slug: string }>();
    const course = useQuery(
        $api.queryOptions('get', '/api/courses/{course_slug}', {
            params: {
                path: { course_slug },
            },
        })
    );

    const runningChallenges = useAtomValue(RunningChallenges);

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
                <SidebarGroup className="pb-1">
                    <div className="-ml-0.5">
                        <Button variant="link" size="sm" asChild>
                            <Link href="/lab">
                                <IconArrowNarrowLeft />
                                Back
                            </Link>
                        </Button>
                    </div>
                    <div className="px-2 mt-1.5">
                        <h3 className="font-semibold">{course?.data?.name}</h3>
                    </div>
                </SidebarGroup>
                <NavMain
                    items={[
                        {
                            title: 'Overview',
                            url: `/lab/courses/${course_slug}`,
                            icon: IconBook,
                        },
                    ]}
                />
                <NavMain
                    title="Lessons"
                    items={
                        course.data?.items.map((item, i) => {
                            const challenge = runningChallenges?.find((c) => c._id === item._id);
                            return {
                                icon: item.type === 'lesson' ? IconBook2 : IconBox,
                                title: item.name,
                                url: `/lab/courses/${course_slug}/${item.type}s/${item.slug}`,
                                rightSection: challenge ? (
                                    <>
                                        <div className="flex-1"></div>
                                        <div
                                            className={clsx('size-2 rounded-full', {
                                                'bg-green-400': challenge.status === 'running',
                                                'bg-orange-400':
                                                    challenge.status === 'starting' ||
                                                    challenge.status === 'stopping',
                                            })}
                                        ></div>
                                    </>
                                ) : undefined,
                            };
                        }) || []
                    }
                />
            </SidebarContent>
            <SidebarFooter>
                <RunningChallengesView />
                <NavSecondary items={navSecondary} className="px-0" />
                <NavUser />
            </SidebarFooter>
        </Sidebar>
    );
}
