import { AdditionalParams } from '.';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export function useStartChallenge(params?: AdditionalParams) {
    const mutation = $api.useMutation('post', '/api/challenges/{challenge_slug}/start', {
        onSuccess: (data, options) => {
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to start challenge', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}

export function useStopChallenge(params?: AdditionalParams) {
    const mutation = $api.useMutation('post', '/api/challenges/{challenge_slug}/stop', {
        onSuccess: (data, options) => {
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to stop challenge', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}

export function useExtendChallenge(params?: AdditionalParams) {
    const mutation = $api.useMutation('post', '/api/challenges/{challenge_slug}/extend', {
        onSuccess: (data, options) => {
            toast.success('Challenge has been extended');
            params?.onSuccess?.();
        },
        onError: (error) => {
            toast.error('Failed to extend challenge', {
                description: error.error,
            });
            params?.onError?.();
        },
    });

    return mutation;
}
