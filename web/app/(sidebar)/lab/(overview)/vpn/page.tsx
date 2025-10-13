'use client';

import { VpnDevice } from '@/types/server/VpnDevice';
import { DataTable } from '@components/data-table';
import { PageHeader } from '@components/page-header';
import { Button } from '@components/ui/button';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from '@components/ui/dropdown-menu';
import { RunningChallenges } from '@lib/atoms';
import { VpnDevices } from '@lib/atoms/vpn-devices';
import { IconChevronDown, IconCopy, IconDevicesPlus, IconPlus } from '@tabler/icons-react';
import { ColumnDef } from '@tanstack/react-table';
import clsx from 'clsx';
import { useAtomValue } from 'jotai';
import moment from 'moment';
import Link from 'next/link';
import { useMemo } from 'react';

export default function Page() {
    // TODO: Fix tables layout
    const devices = useAtomValue(VpnDevices);
    const challenges = useAtomValue(RunningChallenges);

    const userDevices = useMemo(() => devices?.filter((d) => d.type === 'client'), [devices]);
    const userChallenges = useMemo(
        () => devices?.filter((d) => d.type === 'challenge' && d.online),
        [devices]
    );

    const columns: ColumnDef<VpnDevice>[] = useMemo(
        () => [
            {
                size: 150,
                header: 'Name',
                cell: ({ row }) => {
                    const name =
                        row.original.type === 'challenge'
                            ? challenges?.find((c) => c.hostname === row.original.hostname)?.name
                            : row.original.name;

                    return (
                        <p>
                            <span className="font-medium">{name}</span>
                            {row.original.type === 'challenge' && (
                                <span className="text-muted-foreground ml-2">
                                    {row.original.name}
                                </span>
                            )}
                        </p>
                    );
                },
            },
            {
                size: 150,
                header: 'Addresses',
                cell: ({ row }) => {
                    const addresses = [...row.original.ip_addresses, row.original.hostname];

                    return (
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <div className="flex gap-1.5 items-center cursor-pointer">
                                    <p className="font-mono">{row.original.ip_addresses?.[0]}</p>
                                    <Button
                                        variant="secondary"
                                        size="icon-xs"
                                        className="rounded-full size-5"
                                    >
                                        <IconChevronDown />
                                    </Button>
                                </div>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="start">
                                {addresses.map((a) => (
                                    <DropdownMenuItem key={a}>
                                        {a}
                                        <div className="flex-1"></div>
                                        <IconCopy />
                                    </DropdownMenuItem>
                                ))}
                            </DropdownMenuContent>
                        </DropdownMenu>
                    );
                },
            },
            {
                size: 150,
                header: 'Last seen',
                cell: ({ row }) => (
                    <div className="flex items-center gap-1.5">
                        <div
                            className={clsx('size-2 rounded-full', {
                                'bg-green-500': row.original.online,
                                'bg-muted-foreground': !row.original.online,
                            })}
                        ></div>
                        <p className="text-muted-foreground font-medium">
                            {row.original.online
                                ? 'Connected'
                                : moment(row.original.last_seen).fromNow()}
                        </p>
                    </div>
                ),
            },
        ],
        [challenges]
    );

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'VPN',
                    },
                ]}
                rightSection={
                    <Button asChild>
                        <Link href="/lab/vpn/setup">
                            <IconDevicesPlus />
                            Connect a Device
                        </Link>
                    </Button>
                }
            />
            <div className="p-4 pt-0">
                <h2 className="text-lg font-medium mb-2.5">Your Devices</h2>
                <DataTable columns={columns} data={userDevices ?? []} />
                <h2 className="text-lg font-medium mb-2.5 mt-3">Your Challenges</h2>
                <DataTable columns={columns} data={userChallenges ?? []} />
            </div>
        </>
    );
}
