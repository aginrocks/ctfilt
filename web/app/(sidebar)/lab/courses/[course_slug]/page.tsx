'use client';
import MarkdownRenderer from '@components/markdown';
import { PageHeader } from '@components/page-header';
import { Tag } from '@components/ui/tag';
import { $api } from '@lib/providers/api';
import { IconBook2, IconBox, IconBrain } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'next/navigation';

function capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

export default function Page() {
    const { course_slug } = useParams<{ course_slug: string }>();

    const course = useQuery(
        $api.queryOptions('get', '/api/courses/{course_slug}', {
            params: {
                path: { course_slug },
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
                    },
                ]}
            />
            <div className="flex justify-center p-6">
                <div className="max-w-2xl w-full">
                    <h1 className="scroll-m-20 text-4xl font-bold tracking-tight text-balance [&:not(:first-child)]:mt-6 mb-4">
                        {course.data?.name}
                    </h1>
                    <div className="flex flex-wrap gap-2 mb-1">
                        <Tag>
                            <IconBrain />
                            {course.data && capitalize(course.data.difficulty)}
                        </Tag>
                        <Tag>
                            <IconBook2 />
                            {course.data?.items.filter((item) => item.type === 'lesson')?.length ||
                                0}{' '}
                            Lessons
                        </Tag>
                        <Tag>
                            <IconBox />
                            {course.data?.items.filter((item) => item.type === 'challenge')
                                ?.length || 0}{' '}
                            Challenges
                        </Tag>
                        {course.data?.tags?.map((t, i) => (
                            <Tag key={i}>{t}</Tag>
                        ))}
                    </div>
                    <p className="text-muted-foreground">{course?.data?.description}</p>
                    <div className="px-4 py-3.5 mt-4 border rounded-md">
                        <h3 className="text-lg font-semibold">This course includes</h3>
                        <ul className="ml-1 mt-0.5 list-disc list-inside">
                            {course.data?.objectives?.map((obj, i) => (
                                <li key={i}>{obj}</li>
                            ))}
                        </ul>
                    </div>
                </div>
            </div>
        </>
    );
}
