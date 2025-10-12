'use client';
import {
    IconBrandAndroid,
    IconBrandApple,
    IconBrandUbuntu,
    IconBrandWindows,
    IconExternalLink,
    IconSnowflake,
    IconWorld,
} from '@tabler/icons-react';
import { Step } from './step';
import { Setting } from '@components/settings/setting';
import { useForm } from 'react-hook-form';
import z from 'zod';
import { Form } from '@components/ui/form';
import { Button } from '@components/ui/button';
import Link from 'next/link';
import { CodeBlock } from '@components/ui/codeblock';
import Image from 'next/image';
import { InlineCode } from '@components/markdown';
import { ConfirmConnection } from './confirm-connection';

const formSchema = z.object({
    os: z.enum(['windows', 'macos', 'linux', 'android']),
});

export default function VpnOnboarding() {
    const form = useForm<z.infer<typeof formSchema>>();
    const os = form.watch('os');

    return (
        <Form {...form}>
            <div>
                <IconWorld className="size-8 mb-3 text-muted-foreground" />
                <h1 className="text-3xl font-bold mb-2">Let's set up your VPN connection</h1>
                <p className="text-muted-foreground font-medium mb-4">
                    Set up your VPN to access CTF challenges and contests within a secure, isolated
                    network.
                </p>
                <Setting
                    name="os"
                    type="select"
                    layout="vertical"
                    formControl={form.control}
                    className="mb-5"
                    options={[
                        {
                            icon: IconBrandUbuntu,
                            label: 'Linux',
                            value: 'linux',
                        },
                        {
                            icon: IconBrandApple,
                            label: 'macOS',
                            value: 'macos',
                        },
                        {
                            icon: IconBrandWindows,
                            label: 'Windows',
                            value: 'windows',
                        },
                        {
                            icon: IconBrandAndroid,
                            label: 'Android',
                            value: 'android',
                        },
                    ]}
                />
                <Step number={1} title="Install Tailscale">
                    <div className="mt-2.5 mb-3">
                        {os === 'linux' && (
                            <CodeBlock
                                language="text"
                                code="curl -fsSL https://tailscale.com/install.sh | sh"
                            />
                        )}
                        {os === 'macos' && (
                            <>
                                <Button asChild>
                                    <Link href="https://pkgs.tailscale.com/stable/Tailscale-latest-macos.pkg">
                                        Download for macOS
                                    </Link>
                                </Button>
                                <div className="text-sm text-muted-foreground mt-2.5">
                                    Requires macOS Big Sur 11.0 or later.
                                    <br />
                                    You can also download Tailscale on the{' '}
                                    <a
                                        href="https://apps.apple.com/ca/app/tailscale/id1475387142"
                                        className="text-primary"
                                        target="_blank"
                                    >
                                        macOS App Store
                                    </a>
                                    .
                                </div>
                            </>
                        )}
                        {os === 'windows' && (
                            <>
                                <Button asChild>
                                    <Link href="https://pkgs.tailscale.com/stable/tailscale-setup-latest.exe">
                                        Download for Windows
                                    </Link>
                                </Button>
                                <div className="text-sm text-muted-foreground mt-2.5">
                                    Requires Windows 10 or later.
                                </div>
                            </>
                        )}
                        {os === 'android' && (
                            <>
                                <Button asChild>
                                    <Link href="https://play.google.com/store/apps/details?id=com.tailscale.ipn">
                                        Download for Android
                                    </Link>
                                </Button>
                                <div className="text-sm text-muted-foreground mt-2.5">
                                    Requires Android 8 or later.
                                </div>
                            </>
                        )}
                    </div>
                </Step>
                <Step number={2} title="Authenticate with your account">
                    <div className="mb-3">
                        {os === 'linux' && (
                            <>
                                <p className="text-muted-foreground mt-1">
                                    In order to use the VPN, you'll need to provide an alternate
                                    Tailscale server URL. To do so, use the follwowing command:
                                </p>
                                <CodeBlock
                                    language="text"
                                    code="sudo tailscale up --login-server https://vpn.ctf.agin.rocks"
                                />
                                <p className="text-muted-foreground mt-1">
                                    Open the provided link and log in using your CTFILT account.
                                </p>
                            </>
                        )}
                        {os === 'macos' && (
                            <>
                                <p className="text-muted-foreground mt-1">
                                    In order to use the VPN, you'll need to provide an alternate
                                    Tailscale server URL. To do so, open Tailscale settings and
                                    select the arrow next to <b>Add Account...</b>
                                </p>
                                <p className="text-muted-foreground mt-2">
                                    Then, enter <InlineCode>https://vpn.ctf.agin.rocks</InlineCode>{' '}
                                    as the server URL.
                                </p>
                                <Image
                                    src="/images/tailscale-macos.png"
                                    width={500}
                                    height={600}
                                    alt=""
                                />
                                <p className="text-muted-foreground">
                                    Click <b>Add Account...</b> and log in using your CTFILT
                                    account.
                                </p>
                            </>
                        )}
                    </div>
                </Step>
                <Step number={3} title="Confirm the Connection">
                    <ConfirmConnection />
                </Step>
            </div>
        </Form>
    );
}
