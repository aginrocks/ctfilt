'use client';

import React from 'react';

import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from '@/components/ui/sidebar';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import Link from 'next/link';

// TODO: Add progress indicators
export function LabOverviewNav({ ...props }: React.ComponentPropsWithoutRef<typeof SidebarGroup>) {
    const { data } = useQuery($api.queryOptions('get', '/api/courses'));
    return (
        <SidebarGroup {...props}>
            <SidebarGroupLabel>Courses</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    {data?.map((item) => (
                        <SidebarMenuItem key={item._id}>
                            <SidebarMenuButton asChild>
                                <Link href={`/lab/${item.slug}`}>
                                    <span>{item.name}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    ))}
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    );
}
