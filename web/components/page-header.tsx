import { SidebarTrigger } from './ui/sidebar';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from './ui/breadcrumb';
import { Separator } from './ui/separator';
import React from 'react';
import Link from 'next/link';

export type PathSegment = {
    label: string | undefined;
    href?: string;
};

export type PageHeaderProps = {
    path: PathSegment[];
    rightSection?: React.ReactNode;
};

export function PageHeader({ path, rightSection }: PageHeaderProps) {
    const lastSegment = path[path.length - 1];
    const otherSegments = path.slice(0, -1);

    return (
        <header className="flex h-14 shrink-0 items-center gap-2">
            <div className="flex flex-1 items-center gap-2 px-3">
                <SidebarTrigger />
                <Separator
                    orientation="vertical"
                    className="mr-2 data-[orientation=vertical]:h-4"
                />
                <Breadcrumb>
                    <BreadcrumbList>
                        {otherSegments.map((segment, i) => (
                            <React.Fragment key={i}>
                                <BreadcrumbItem className="hidden md:block">
                                    <BreadcrumbLink asChild>
                                        <Link href={segment.href ?? '#'}>{segment.label}</Link>
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator className="hidden md:block" />
                            </React.Fragment>
                        ))}
                        <BreadcrumbItem>
                            <BreadcrumbPage>{lastSegment?.label}</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </div>
            {/*<div className="ml-auto px-3">
                <NavActions />
            </div>*/}
        </header>
    );
}
