'use client';

import { Input } from './ui/input';
import z from 'zod';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from './ui/form';
import { InputGroup, InputGroupAddon, InputGroupInput } from './ui/input-group';
import { IconFlag } from '@tabler/icons-react';

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
    });

    return (
        <Form {...form}>
            <FormField
                control={form.control}
                name="flag"
                render={({ field }) => (
                    <FormItem className="flex-1">
                        <FormControl>
                            <InputGroup className="h-10">
                                <InputGroupInput placeholder="Enter flag" className="font-mono" />
                                <InputGroupAddon>
                                    <IconFlag />
                                </InputGroupAddon>
                                <InputGroupAddon align="inline-end">
                                    {/*<InputGroupButton>Search</InputGroupButton>*/}
                                </InputGroupAddon>
                            </InputGroup>
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                )}
            />
        </Form>
    );
}
