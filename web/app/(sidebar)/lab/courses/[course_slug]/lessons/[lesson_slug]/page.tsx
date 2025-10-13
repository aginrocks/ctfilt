'use client';
import MarkdownRenderer from '@components/markdown';
import { PageHeader } from '@components/page-header';
import { useSaveLast } from '@lib/hooks';
import { $api } from '@lib/providers/api';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';

export default function Page() {
    const { course_slug, lesson_slug } = useParams<{ course_slug: string; lesson_slug: string }>();
    useSaveLast(course_slug, lesson_slug, 'lesson');

    const course = useQuery(
        $api.queryOptions('get', '/api/courses/{course_slug}', {
            params: {
                path: { course_slug },
            },
        })
    );

    const lesson = useQuery(
        $api.queryOptions('get', '/api/courses/{course_slug}/lessons/{lesson_slug}', {
            params: {
                path: { course_slug, lesson_slug },
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
                        label: lesson.data?.name,
                    },
                ]}
            />
            <div className="flex justify-center p-6">
                <div className="max-w-2xl w-full">
                    <MarkdownRenderer>{lesson.data?.content || null}</MarkdownRenderer>
                </div>
            </div>
        </>
    );
}
