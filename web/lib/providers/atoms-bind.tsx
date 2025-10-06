'use client';
import { useBindAtoms } from '@lib/atoms';

export type AtomsBindProviderProps = {
    children?: React.ReactNode;
};

export function AtomsBindProvider({ children }: AtomsBindProviderProps) {
    useBindAtoms();

    return children;
}
