import { VpnDevice } from '@/types/server/VpnDevice';
import { useSocketEvent } from '@lib/hooks';
import { atom, useAtomValue, useSetAtom } from 'jotai';
import { useCallback, useMemo } from 'react';

export const VpnDevices = atom<VpnDevice[]>();

export function useBindVPN() {
    const setVPNDevices = useSetAtom(VpnDevices);

    useSocketEvent(
        'vpn-state',
        useCallback((message) => {
            setVPNDevices(message.devices);
        }, [])
    );
}

// Returns info about a device that was connected since the hook has been mounted
export function useRecentlyConnected() {
    const devices = useAtomValue(VpnDevices);
    const device = useMemo(
        () =>
            devices
                ?.filter((d) => d.online && d.type === 'client')
                .sort(
                    (a, b) =>
                        new Date(b.created_at ?? '').getTime() -
                        new Date(a.created_at ?? '').getTime()
                )[0],
        [devices]
    );

    return device;
}
