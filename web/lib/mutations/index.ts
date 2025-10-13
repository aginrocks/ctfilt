export * from './challenge';
export * from './course';

export type AdditionalParams = {
    onSuccess?: () => void;
    onError?: () => void;
};
