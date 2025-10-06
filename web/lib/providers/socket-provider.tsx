import { ServerMessage } from '@/types/server/ServerMessage';
import { createContext } from 'react';
import useWebSocket from 'react-use-websocket';
import { WebSocketHook } from 'react-use-websocket/dist/lib/types';

export type SocketProviderProps = {
    children: React.ReactNode;
};

export const WebSocketContext = createContext<WebSocketHook<ServerMessage> | null>(null);

export function SocketProvider({ children }: SocketProviderProps) {
    const socket = useWebSocket<ServerMessage>('/api/ws');
    return <WebSocketContext.Provider value={null}>{children}</WebSocketContext.Provider>;
}
