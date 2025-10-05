import { paths } from '@/types/api';
import MarkdownRenderer from './markdown';
import { IconBox, IconPlayerPlayFilled } from '@tabler/icons-react';
import { Button } from './ui/button';

export type ChallengeViewProps = {
    challenge: paths['/api/challenges/{challenge_slug}']['get']['responses']['200']['content']['application/json'];
};

export function ChallengeView({ challenge }: ChallengeViewProps) {
    return (
        <div>
            <div className="mb-3">
                <h1 className="scroll-m-20 text-3xl font-bold tracking-tight text-balance mb-2">
                    {challenge.name}
                </h1>
                <p className="italic font-medium text-muted-foreground">{challenge.description}</p>
            </div>
            <MarkdownRenderer>{challenge.details}</MarkdownRenderer>
            <div className="mt-4">
                <Button size="lg">
                    <IconPlayerPlayFilled /> Start Challenge
                </Button>
            </div>
        </div>
    );
}
