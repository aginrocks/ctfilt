import { RunningChallenge } from '@/types/server/RunningChallenge';
import { atom } from 'jotai';

export const RunningChallenges = atom<RunningChallenge[]>();
