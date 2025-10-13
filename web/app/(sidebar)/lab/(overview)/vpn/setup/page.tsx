import { PageHeader } from '@components/page-header';
import VpnOnboarding from './onboarding';

export default function Page() {
    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'VPN',
                        href: '/lab/vpn',
                    },
                    {
                        label: 'Setup',
                    },
                ]}
            />

            <div className="flex justify-center pt-4 p-6">
                <div className="max-w-2xl w-full">
                    <VpnOnboarding />
                </div>
            </div>
        </>
    );
}
