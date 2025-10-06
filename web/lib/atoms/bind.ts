'use client';

import { useBindChallenges } from './running-challenges';
import { useBindSocket } from './socket';

export function useBindAtoms() {
    useBindSocket();
    useBindChallenges();
}
