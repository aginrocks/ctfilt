'use client';

import { type LucideIcon } from 'lucide-react';

import { SidebarGroup, SidebarGroupLabel, SidebarMenu } from '@/components/ui/sidebar';
import { Icon } from '@tabler/icons-react';

import { NavMainItemComponent } from './nav-main-item';

export type NavMainSubItem = {
    title: string;
    url: string;
};

export type NavMainItem = {
    title: string;
    url: string;
    icon: LucideIcon | Icon;
    isActive?: boolean;
    defaultOpen?: boolean;
    items?: NavMainSubItem[];
    rightSection?: React.ReactNode;
};

export type NavMainProps = {
    title?: string;
    items: NavMainItem[];
};

export function NavMain({ items, title }: NavMainProps) {
    return (
        <SidebarGroup>
            {title && <SidebarGroupLabel>{title}</SidebarGroupLabel>}
            <SidebarMenu>
                {items.map((item) => (
                    <NavMainItemComponent key={item.url} item={item} />
                ))}
            </SidebarMenu>
        </SidebarGroup>
    );
}
