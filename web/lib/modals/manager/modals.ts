'use client';
import { Challenge } from '../challenge';
import { ModalComponentBindings, ModalDefinition } from './types';

export type Modals = {
    Challenge: ModalDefinition<{
        payload: {
            slug: string;
        };
        returnValue: undefined;
    }>;
};

export const ModalsBinding: ModalComponentBindings = {
    Challenge: Challenge,
};
