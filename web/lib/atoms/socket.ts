import { atom, useSetAtom } from 'jotai';
import { useEffect } from 'react';
import ReconnectingWebSocket from 'reconnecting-websocket';

export const SocketAtom = atom<ReconnectingWebSocket>();

export function useBindSocket() {
    const setSocket = useSetAtom(SocketAtom);
    useEffect(() => {
        const socket = new ReconnectingWebSocket('/api/ws');
        setSocket(socket);

        return () => {
            socket.close();
        };
    }, []);
}
