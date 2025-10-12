import { PageHeader } from '@components/page-header';
import VpnOnboarding from './onboarding';

export default function Page() {
    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'VPN',
                    },
                ]}
            />

            <div className="flex justify-center py-4 px-6">
                <div className="max-w-2xl w-full">
                    <VpnOnboarding />
                </div>
            </div>
        </>
    );
}
