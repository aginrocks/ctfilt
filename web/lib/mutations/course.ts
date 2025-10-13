import { $api } from '@lib/providers/api';
import { AdditionalParams } from '.';
import { toast } from 'sonner';

export function useSaveLastMutation(params?: AdditionalParams) {
    const mutation = $api.useMutation('put', '/api/courses/{course_slug}/last', {
        onSuccess: (data, options) => {
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to save last viewed item', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}
