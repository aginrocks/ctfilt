'use client';
import { ServerMessage } from '@/types/server/ServerMessage';
import { SocketAtom } from '@lib/atoms';
import { useAtomValue } from 'jotai';
import { useContext, useEffect } from 'react';

type ServerEvent = ServerMessage['type'];

type PayloadOf<T extends ServerEvent> = Extract<ServerMessage, { type: T }>;

export function useSocketEvent<T extends ServerEvent>(
    event: T,
    callback: (payload: PayloadOf<T>) => void
) {
    const socket = useAtomValue(SocketAtom);

    useEffect(() => {
        if (!socket) return;

        // TODO: Optimize
        const onMessage = (message: MessageEvent) => {
            const data: ServerMessage = JSON.parse(message.data);
            console.log(message);
            if (data.type === event) {
                callback(data as PayloadOf<T>);
            }
        };
        socket.addEventListener('message', onMessage);

        return () => {
            socket.removeEventListener('message', onMessage);
        };
    }, [socket]);
}
