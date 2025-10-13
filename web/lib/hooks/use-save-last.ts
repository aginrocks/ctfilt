import { useSaveLastMutation } from '@lib/mutations';
import { useEffect } from 'react';

export type SaveLastProps = {
    course_slug: string;
    item_slug: string;
    item_type: 'lesson' | 'challenge';
};

export function useSaveLast(
    course_slug: string,
    item_slug: string,
    item_type: 'lesson' | 'challenge'
) {
    const saveLast = useSaveLastMutation();

    useEffect(() => {
        saveLast.mutate({
            body: {
                slug: item_slug,
                type: item_type,
            },
            params: {
                path: { course_slug },
            },
        });
    }, [course_slug, item_type]);
}
