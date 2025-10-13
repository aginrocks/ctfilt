'use client';

import z from 'zod';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Form, FormControl, FormField, FormItem, FormMessage } from './ui/form';
import { InputGroup, InputGroupAddon, InputGroupInput } from './ui/input-group';
import { IconFlag, IconSend } from '@tabler/icons-react';
import { Button } from './ui/button';
import { $api } from '@lib/providers/api';
import { toast } from 'sonner';

export type FlagInputProps = {
    challengeSlug: string;
};

const formSchema = z.object({
    flag: z.string(),
});

type FormSchema = z.infer<typeof formSchema>;

export function FlagInput({ challengeSlug }: FlagInputProps) {
    const form = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            flag: '',
        },
    });

    const mutation = $api.useMutation('post', '/api/challenges/{challenge_slug}/submit', {
        onSuccess: (data, options) => {
            if (data.correct) {
                toast.success('Correct flag');
            } else {
                toast.error('Incorrect flag');
                form.setError('flag', { message: 'Incorrect flag' });
            }
        },
        onError: (error) => {
            toast.error('Failed to submit flag', {
                description: error.error,
            });
        },
    });

    return (
        <Form {...form}>
            <form
                className="flex-1 flex gap-2.5"
                onSubmit={form.handleSubmit((data) =>
                    mutation.mutate({
                        body: data,
                        params: {
                            path: { challenge_slug: challengeSlug },
                        },
                    })
                )}
            >
                <FormField
                    control={form.control}
                    name="flag"
                    render={({ field }) => (
                        <FormItem className="flex-1">
                            <FormControl>
                                <InputGroup className="h-10">
                                    <InputGroupInput
                                        placeholder="Enter flag"
                                        className="font-mono"
                                        {...field}
                                    />
                                    <InputGroupAddon>
                                        <IconFlag />
                                    </InputGroupAddon>
                                </InputGroup>
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button size="icon-lg" variant="default">
                    <IconSend />
                </Button>
            </form>
        </Form>
    );
}
