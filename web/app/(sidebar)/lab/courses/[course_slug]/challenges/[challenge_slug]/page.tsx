'use client';
import { ChallengeView } from '@components/challenge-view';
import MarkdownRenderer from '@components/markdown';
import { PageHeader } from '@components/page-header';
import { $api } from '@lib/providers/api';
import { IconBox } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';

export default function Page() {
    const { course_slug, challenge_slug } = useParams<{
        course_slug: string;
        challenge_slug: string;
    }>();

    const course = useQuery(
        $api.queryOptions('get', '/api/courses/{course_slug}', {
            params: {
                path: { course_slug },
            },
        })
    );

    const challenge = useQuery(
        $api.queryOptions('get', '/api/challenges/{challenge_slug}', {
            params: {
                path: { challenge_slug },
            },
        })
    );

    return (
        <>
            <PageHeader
                path={[
                    {
                        label: 'Lab',
                        href: '/lab',
                    },
                    {
                        label: course.data?.name,
                        href: `/lab/courses/${course_slug}`,
                    },
                    {
                        label: challenge.data?.name,
                    },
                ]}
            />
            <div className="flex justify-center p-6">
                <div className="max-w-2xl w-full rounded-lg border shadow-md p-6">
                    <div className="px-3.5 py-2 rounded-full bg-secondary/50 text-secondary-foreground text-sm font-medium max-w-max mb-3 flex items-center gap-1.5">
                        <IconBox className="size-4" />
                        Challenge
                    </div>
                    {challenge.data && <ChallengeView challenge={challenge.data} />}
                </div>
            </div>
        </>
    );
}
