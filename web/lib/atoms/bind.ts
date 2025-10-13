'use client';

import { useBindChallenges } from './running-challenges';
import { useBindSocket } from './socket';
import { useBindVPN } from './vpn-devices';

export function useBindAtoms() {
    useBindSocket();
    useBindChallenges();
    useBindVPN();
}
