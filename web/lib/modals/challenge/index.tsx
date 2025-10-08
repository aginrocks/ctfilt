'use client';
import { Dialog, DialogContent } from '@components/ui/dialog';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import * as React from 'react';
import { ModalProps } from '../manager';
import { useQuery } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { ChallengeView } from '@components/challenge-view';
import { VisuallyHidden } from '@radix-ui/react-visually-hidden';
import { DialogTitle } from '@radix-ui/react-dialog';

export function Challenge({
    payload,
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & ModalProps<'Challenge'>) {
    const challenge = useQuery(
        $api.queryOptions('get', '/api/challenges/{challenge_slug}', {
            params: {
                path: { challenge_slug: payload?.slug || '' },
            },
        })
    );

    return (
        <Dialog {...props}>
            <DialogContent className="sm:max-w-xl">
                <VisuallyHidden>
                    <DialogTitle>{challenge.data?.name}</DialogTitle>
                </VisuallyHidden>
                {challenge.data && <ChallengeView challenge={challenge.data} />}
            </DialogContent>
        </Dialog>
    );
}
