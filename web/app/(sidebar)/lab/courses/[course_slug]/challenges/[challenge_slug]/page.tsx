'use client';
import { ChallengeView } from '@components/challenge-view';
import MarkdownRenderer from '@components/markdown';
import { PageHeader } from '@components/page-header';
import { Tag } from '@components/ui/tag';
import { useSaveLast } from '@lib/hooks';
import { $api } from '@lib/providers/api';
import { IconBox } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';

export default function Page() {
    const { course_slug, challenge_slug } = useParams<{
        course_slug: string;
        challenge_slug: string;
    }>();
    useSaveLast(course_slug, challenge_slug, 'challenge');

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
                    <Tag>
                        <IconBox />
                        Challenge
                    </Tag>
                    {challenge.data && <ChallengeView challenge={challenge.data} />}
                </div>
            </div>
        </>
    );
}
